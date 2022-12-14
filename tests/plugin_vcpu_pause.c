/*
 * Copyright 2017 The ChromiumOS Authors
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#include <errno.h>
#include <fcntl.h>
#include <linux/memfd.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/eventfd.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <time.h>
#include <unistd.h>

#include "crosvm.h"

#ifndef F_LINUX_SPECIFIC_BASE
#define F_LINUX_SPECIFIC_BASE 1024
#endif

#ifndef F_ADD_SEALS
#define F_ADD_SEALS (F_LINUX_SPECIFIC_BASE + 9)
#endif

#ifndef F_SEAL_SHRINK
#define F_SEAL_SHRINK 0x0002
#endif

#define SERIAL_ADDRESS 0x3f8
#define KILL_ADDRESS 0x3f9

static char g_serial_out[16];
static int g_next_evt;
static int g_kill_evt;

static bool g_paused;
static bool g_exit_loop;
static pthread_mutex_t g_pause_mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t g_pause_cond = PTHREAD_COND_INITIALIZER;

static volatile int count;

static void *vcpu_thread_fn(void *arg) {
    struct crosvm_vcpu *vcpu = arg;
    struct crosvm_vcpu_event evt;
    int i = 0;

    while (crosvm_vcpu_wait(vcpu, &evt) == 0) {
        if (evt.kind == CROSVM_VCPU_EVENT_KIND_INIT) {
            struct kvm_sregs sregs;
            crosvm_vcpu_get_sregs(vcpu, &sregs);
            sregs.cs.base = 0;
            sregs.cs.selector = 0;
            sregs.es.base = KILL_ADDRESS;
            sregs.es.selector = 0;
            crosvm_vcpu_set_sregs(vcpu, &sregs);

            struct kvm_regs regs;
            crosvm_vcpu_get_regs(vcpu, &regs);
            regs.rip = 0x1000;
            regs.rax = 2;
            regs.rbx = 7;
            regs.rflags = 2;
            crosvm_vcpu_set_regs(vcpu, &regs);

            /* Signal the main thread that init is done */
            uint64_t dummy = 1;
            write(g_next_evt, &dummy, sizeof(dummy));
        }
        else if (evt.kind == CROSVM_VCPU_EVENT_KIND_IO_ACCESS &&
                 evt.io_access.address_space == CROSVM_ADDRESS_SPACE_IOPORT &&
                 evt.io_access.address == KILL_ADDRESS &&
                 evt.io_access.is_write &&
                 evt.io_access.length == 1 &&
                 evt.io_access.data[0] == 1) {
            uint64_t dummy = 1;
            write(g_kill_evt, &dummy, sizeof(dummy));
            return NULL;
        }
        else if (evt.kind == CROSVM_VCPU_EVENT_KIND_PAUSED) {
            /* Signal that we paused */
            uint64_t dummy = 1;
            write(g_next_evt, &dummy, sizeof(dummy));

            /* Wait till we can continue again */
            pthread_mutex_lock(&g_pause_mutex);
            while (g_paused)
                pthread_cond_wait(&g_pause_cond, &g_pause_mutex);

            /* Kick the VM from infinite loop if requested */
            if (g_exit_loop) {
                struct kvm_regs regs;
                crosvm_vcpu_get_regs(vcpu, &regs);
                regs.rbx = 1;
                crosvm_vcpu_set_regs(vcpu, &regs);
            }

            /* Signal that we are no longer paused */
            write(g_next_evt, &dummy, sizeof(dummy));

            pthread_mutex_unlock(&g_pause_mutex);
        }
        crosvm_vcpu_resume(vcpu);
    }

    return NULL;
}

static int signal_pause(struct crosvm *crosvm) {
    pthread_mutex_lock(&g_pause_mutex);
    g_paused = true;
    pthread_mutex_unlock(&g_pause_mutex);

    return crosvm_pause_vcpus(crosvm, 1, NULL);
}

static void signal_unpause(struct crosvm *crosvm, bool exit_loop) {
    pthread_mutex_lock(&g_pause_mutex);
    g_paused = false;
    g_exit_loop = exit_loop;
    pthread_cond_broadcast(&g_pause_cond);
    pthread_mutex_unlock(&g_pause_mutex);
}

int main(int argc, char** argv) {
    const uint8_t code[] = {
    /*
    0000  00D8    add al,bl
    0002  80FB01  cmp bl, 0x1
    0005  75F9    jne 0x0
    0007  BAF903  mov dx,0x3f8
    000A  B001    mov al,0x1
    000C  EE      out dx,al
    000D  F4      hlt
    */
        0x00, 0xd8,
        0x80, 0xfb, 0x01,
        0x75, 0xf9,
        0xba, 0xf9, 0x03,
        0xb0, 0x01,
        0xee,
        0xf4
    };

    g_next_evt = eventfd(0, 0);
    if (g_next_evt == -1) {
        fprintf(stderr, "failed to create eventfd: %d\n", errno);
        return 1;
    }

    struct crosvm *crosvm;
    int ret = crosvm_connect(&crosvm);
    if (ret) {
        fprintf(stderr, "failed to connect to crosvm: %d\n", ret);
        return 1;
    }

    /* We needs this eventfd to know when to exit before being killed. */
    g_kill_evt = crosvm_get_shutdown_eventfd(crosvm);
    if (g_kill_evt < 0) {
        fprintf(stderr, "failed to get kill eventfd: %d\n", g_kill_evt);
        return 1;
    }

    ret = crosvm_reserve_range(crosvm, CROSVM_ADDRESS_SPACE_IOPORT,
                               KILL_ADDRESS, 1);
    if (ret) {
        fprintf(stderr, "failed to reserve mmio range: %d\n", ret);
        return 1;
    }

    int mem_size = 0x2000;
    int mem_fd = syscall(SYS_memfd_create,
                         "guest_mem", MFD_CLOEXEC | MFD_ALLOW_SEALING);
    if (mem_fd < 0) {
        fprintf(stderr, "failed to create guest memfd: %d\n", errno);
        return 1;
    }
    ret = ftruncate(mem_fd, mem_size);
    if (ret) {
        fprintf(stderr, "failed to set size of guest memory: %d\n", errno);
        return 1;
    }
    uint8_t *mem = mmap(NULL, mem_size, PROT_READ | PROT_WRITE, MAP_SHARED,
                        mem_fd, 0x1000);
    if (mem == MAP_FAILED) {
        fprintf(stderr, "failed to mmap guest memory: %d\n", errno);
        return 1;
    }
    fcntl(mem_fd, F_ADD_SEALS, F_SEAL_SHRINK);
    memcpy(mem, code, sizeof(code));

    struct crosvm_memory *mem_obj;
    ret = crosvm_create_memory(crosvm, mem_fd, 0x1000, 0x1000, 0x1000,
                               false, false, &mem_obj);
    if (ret) {
        fprintf(stderr, "failed to create memory in crosvm: %d\n", ret);
        return 1;
    }

    /* get and create a thread for the vcpu */
    struct crosvm_vcpu *vcpu;
    ret = crosvm_get_vcpu(crosvm, 0, &vcpu);
    if (ret) {
       fprintf(stderr, "error while getting vcpu: %d\n", ret);
       return 1;
    }

    pthread_t vcpu_thread;
    ret = pthread_create(&vcpu_thread, NULL, vcpu_thread_fn, vcpu);
    if (ret) {
        fprintf(stderr, "failed to createvcpu thread\n");
        return 1;
    }

    ret = crosvm_start(crosvm);
    if (ret) {
        fprintf(stderr, "failed to tell crosvm to start: %d\n", ret);
        return 1;
    }

    /* Wait till VCPU thread tells us that its initialization is done */
    uint64_t dummy;
    read(g_next_evt, &dummy, sizeof(dummy));

    ret = signal_pause(crosvm);
    if (ret) {
        fprintf(stderr, "failed to pause vcpus (1st time): %d\n", ret);
        return 1;
    }

    /* Wait till VCPU thread tells us it is paused */
    read(g_next_evt, &dummy, sizeof(dummy));

    /* Try pausing VCPUs 2nd time to make sure we do not deadlock */
    ret = signal_pause(crosvm);
    if (ret) {
        fprintf(stderr, "failed to pause vcpus (2nd time): %d\n", ret);
        return 1;
    }

    signal_unpause(crosvm, false);

    /* Wait until VCPU thread tells us that it is no longer paused */
    read(g_next_evt, &dummy, sizeof(dummy));

    /*
     * Try pausing VCPUs 3rd time to see if we will miss pause
     * request as we are exiting previous pause.
     */
    ret = signal_pause(crosvm);
    if (ret) {
        fprintf(stderr, "failed to pause vcpus (2nd time): %d\n", ret);
        return 1;
    }

    signal_unpause(crosvm, true);

    /* Wait for crosvm to request that we exit otherwise we will be killed. */
    read(g_kill_evt, &dummy, sizeof(dummy));

    ret = crosvm_destroy_memory(crosvm, &mem_obj);
    if (ret) {
        fprintf(stderr, "failed to destroy memory in crosvm: %d\n", ret);
        return 1;
    }

    ret = crosvm_reserve_range(crosvm, CROSVM_ADDRESS_SPACE_IOPORT,
                               KILL_ADDRESS, 0);
    if (ret) {
        fprintf(stderr, "failed to unreserve mmio range: %d\n", ret);
        return 1;
    }

    return 0;
}

/* automatically generated by tools/bindgen-all-the-things */

#![allow(clippy::missing_safety_doc)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub const __BITS_PER_LONG: u32 = 64;
pub const __FD_SETSIZE: u32 = 1024;
pub const EM_NONE: u32 = 0;
pub const EM_M32: u32 = 1;
pub const EM_SPARC: u32 = 2;
pub const EM_386: u32 = 3;
pub const EM_68K: u32 = 4;
pub const EM_88K: u32 = 5;
pub const EM_486: u32 = 6;
pub const EM_860: u32 = 7;
pub const EM_MIPS: u32 = 8;
pub const EM_MIPS_RS3_LE: u32 = 10;
pub const EM_MIPS_RS4_BE: u32 = 10;
pub const EM_PARISC: u32 = 15;
pub const EM_SPARC32PLUS: u32 = 18;
pub const EM_PPC: u32 = 20;
pub const EM_PPC64: u32 = 21;
pub const EM_SPU: u32 = 23;
pub const EM_ARM: u32 = 40;
pub const EM_SH: u32 = 42;
pub const EM_SPARCV9: u32 = 43;
pub const EM_H8_300: u32 = 46;
pub const EM_IA_64: u32 = 50;
pub const EM_X86_64: u32 = 62;
pub const EM_S390: u32 = 22;
pub const EM_CRIS: u32 = 76;
pub const EM_M32R: u32 = 88;
pub const EM_MN10300: u32 = 89;
pub const EM_OPENRISC: u32 = 92;
pub const EM_ARCOMPACT: u32 = 93;
pub const EM_XTENSA: u32 = 94;
pub const EM_BLACKFIN: u32 = 106;
pub const EM_UNICORE: u32 = 110;
pub const EM_ALTERA_NIOS2: u32 = 113;
pub const EM_TI_C6000: u32 = 140;
pub const EM_HEXAGON: u32 = 164;
pub const EM_NDS32: u32 = 167;
pub const EM_AARCH64: u32 = 183;
pub const EM_TILEPRO: u32 = 188;
pub const EM_MICROBLAZE: u32 = 189;
pub const EM_TILEGX: u32 = 191;
pub const EM_ARCV2: u32 = 195;
pub const EM_RISCV: u32 = 243;
pub const EM_BPF: u32 = 247;
pub const EM_CSKY: u32 = 252;
pub const EM_FRV: u32 = 21569;
pub const EM_ALPHA: u32 = 36902;
pub const EM_CYGNUS_M32R: u32 = 36929;
pub const EM_S390_OLD: u32 = 41872;
pub const EM_CYGNUS_MN10300: u32 = 48879;
pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;
pub const PT_TLS: u32 = 7;
pub const PT_LOOS: u32 = 1610612736;
pub const PT_HIOS: u32 = 1879048191;
pub const PT_LOPROC: u32 = 1879048192;
pub const PT_HIPROC: u32 = 2147483647;
pub const PT_GNU_EH_FRAME: u32 = 1685382480;
pub const PT_GNU_PROPERTY: u32 = 1685382483;
pub const PT_GNU_STACK: u32 = 1685382481;
pub const PN_XNUM: u32 = 65535;
pub const ET_NONE: u32 = 0;
pub const ET_REL: u32 = 1;
pub const ET_EXEC: u32 = 2;
pub const ET_DYN: u32 = 3;
pub const ET_CORE: u32 = 4;
pub const ET_LOPROC: u32 = 65280;
pub const ET_HIPROC: u32 = 65535;
pub const DT_NULL: u32 = 0;
pub const DT_NEEDED: u32 = 1;
pub const DT_PLTRELSZ: u32 = 2;
pub const DT_PLTGOT: u32 = 3;
pub const DT_HASH: u32 = 4;
pub const DT_STRTAB: u32 = 5;
pub const DT_SYMTAB: u32 = 6;
pub const DT_RELA: u32 = 7;
pub const DT_RELASZ: u32 = 8;
pub const DT_RELAENT: u32 = 9;
pub const DT_STRSZ: u32 = 10;
pub const DT_SYMENT: u32 = 11;
pub const DT_INIT: u32 = 12;
pub const DT_FINI: u32 = 13;
pub const DT_SONAME: u32 = 14;
pub const DT_RPATH: u32 = 15;
pub const DT_SYMBOLIC: u32 = 16;
pub const DT_REL: u32 = 17;
pub const DT_RELSZ: u32 = 18;
pub const DT_RELENT: u32 = 19;
pub const DT_PLTREL: u32 = 20;
pub const DT_DEBUG: u32 = 21;
pub const DT_TEXTREL: u32 = 22;
pub const DT_JMPREL: u32 = 23;
pub const DT_ENCODING: u32 = 32;
pub const OLD_DT_LOOS: u32 = 1610612736;
pub const DT_LOOS: u32 = 1610612749;
pub const DT_HIOS: u32 = 1879044096;
pub const DT_VALRNGLO: u32 = 1879047424;
pub const DT_VALRNGHI: u32 = 1879047679;
pub const DT_ADDRRNGLO: u32 = 1879047680;
pub const DT_ADDRRNGHI: u32 = 1879047935;
pub const DT_VERSYM: u32 = 1879048176;
pub const DT_RELACOUNT: u32 = 1879048185;
pub const DT_RELCOUNT: u32 = 1879048186;
pub const DT_FLAGS_1: u32 = 1879048187;
pub const DT_VERDEF: u32 = 1879048188;
pub const DT_VERDEFNUM: u32 = 1879048189;
pub const DT_VERNEED: u32 = 1879048190;
pub const DT_VERNEEDNUM: u32 = 1879048191;
pub const OLD_DT_HIOS: u32 = 1879048191;
pub const DT_LOPROC: u32 = 1879048192;
pub const DT_HIPROC: u32 = 2147483647;
pub const STB_LOCAL: u32 = 0;
pub const STB_GLOBAL: u32 = 1;
pub const STB_WEAK: u32 = 2;
pub const STT_NOTYPE: u32 = 0;
pub const STT_OBJECT: u32 = 1;
pub const STT_FUNC: u32 = 2;
pub const STT_SECTION: u32 = 3;
pub const STT_FILE: u32 = 4;
pub const STT_COMMON: u32 = 5;
pub const STT_TLS: u32 = 6;
pub const EI_NIDENT: u32 = 16;
pub const PF_R: u32 = 4;
pub const PF_W: u32 = 2;
pub const PF_X: u32 = 1;
pub const SHT_NULL: u32 = 0;
pub const SHT_PROGBITS: u32 = 1;
pub const SHT_SYMTAB: u32 = 2;
pub const SHT_STRTAB: u32 = 3;
pub const SHT_RELA: u32 = 4;
pub const SHT_HASH: u32 = 5;
pub const SHT_DYNAMIC: u32 = 6;
pub const SHT_NOTE: u32 = 7;
pub const SHT_NOBITS: u32 = 8;
pub const SHT_REL: u32 = 9;
pub const SHT_SHLIB: u32 = 10;
pub const SHT_DYNSYM: u32 = 11;
pub const SHT_NUM: u32 = 12;
pub const SHT_LOPROC: u32 = 1879048192;
pub const SHT_HIPROC: u32 = 2147483647;
pub const SHT_LOUSER: u32 = 2147483648;
pub const SHT_HIUSER: u32 = 4294967295;
pub const SHF_WRITE: u32 = 1;
pub const SHF_ALLOC: u32 = 2;
pub const SHF_EXECINSTR: u32 = 4;
pub const SHF_RELA_LIVEPATCH: u32 = 1048576;
pub const SHF_RO_AFTER_INIT: u32 = 2097152;
pub const SHF_MASKPROC: u32 = 4026531840;
pub const SHN_UNDEF: u32 = 0;
pub const SHN_LORESERVE: u32 = 65280;
pub const SHN_LOPROC: u32 = 65280;
pub const SHN_HIPROC: u32 = 65311;
pub const SHN_LIVEPATCH: u32 = 65312;
pub const SHN_ABS: u32 = 65521;
pub const SHN_COMMON: u32 = 65522;
pub const SHN_HIRESERVE: u32 = 65535;
pub const EI_MAG0: u32 = 0;
pub const EI_MAG1: u32 = 1;
pub const EI_MAG2: u32 = 2;
pub const EI_MAG3: u32 = 3;
pub const EI_CLASS: u32 = 4;
pub const EI_DATA: u32 = 5;
pub const EI_VERSION: u32 = 6;
pub const EI_OSABI: u32 = 7;
pub const EI_PAD: u32 = 8;
pub const ELFMAG0: u32 = 127;
pub const ELFMAG1: u8 = 69u8;
pub const ELFMAG2: u8 = 76u8;
pub const ELFMAG3: u8 = 70u8;
pub const ELFMAG: &'static [u8; 5usize] = b"\x7FELF\0";
pub const SELFMAG: u32 = 4;
pub const ELFCLASSNONE: u32 = 0;
pub const ELFCLASS32: u32 = 1;
pub const ELFCLASS64: u32 = 2;
pub const ELFCLASSNUM: u32 = 3;
pub const ELFDATANONE: u32 = 0;
pub const ELFDATA2LSB: u32 = 1;
pub const ELFDATA2MSB: u32 = 2;
pub const EV_NONE: u32 = 0;
pub const EV_CURRENT: u32 = 1;
pub const EV_NUM: u32 = 2;
pub const ELFOSABI_NONE: u32 = 0;
pub const ELFOSABI_LINUX: u32 = 3;
pub const ELF_OSABI: u32 = 0;
pub const NT_PRSTATUS: u32 = 1;
pub const NT_PRFPREG: u32 = 2;
pub const NT_PRPSINFO: u32 = 3;
pub const NT_TASKSTRUCT: u32 = 4;
pub const NT_AUXV: u32 = 6;
pub const NT_SIGINFO: u32 = 1397311305;
pub const NT_FILE: u32 = 1179208773;
pub const NT_PRXFPREG: u32 = 1189489535;
pub const NT_PPC_VMX: u32 = 256;
pub const NT_PPC_SPE: u32 = 257;
pub const NT_PPC_VSX: u32 = 258;
pub const NT_PPC_TAR: u32 = 259;
pub const NT_PPC_PPR: u32 = 260;
pub const NT_PPC_DSCR: u32 = 261;
pub const NT_PPC_EBB: u32 = 262;
pub const NT_PPC_PMU: u32 = 263;
pub const NT_PPC_TM_CGPR: u32 = 264;
pub const NT_PPC_TM_CFPR: u32 = 265;
pub const NT_PPC_TM_CVMX: u32 = 266;
pub const NT_PPC_TM_CVSX: u32 = 267;
pub const NT_PPC_TM_SPR: u32 = 268;
pub const NT_PPC_TM_CTAR: u32 = 269;
pub const NT_PPC_TM_CPPR: u32 = 270;
pub const NT_PPC_TM_CDSCR: u32 = 271;
pub const NT_PPC_PKEY: u32 = 272;
pub const NT_386_TLS: u32 = 512;
pub const NT_386_IOPERM: u32 = 513;
pub const NT_X86_XSTATE: u32 = 514;
pub const NT_S390_HIGH_GPRS: u32 = 768;
pub const NT_S390_TIMER: u32 = 769;
pub const NT_S390_TODCMP: u32 = 770;
pub const NT_S390_TODPREG: u32 = 771;
pub const NT_S390_CTRS: u32 = 772;
pub const NT_S390_PREFIX: u32 = 773;
pub const NT_S390_LAST_BREAK: u32 = 774;
pub const NT_S390_SYSTEM_CALL: u32 = 775;
pub const NT_S390_TDB: u32 = 776;
pub const NT_S390_VXRS_LOW: u32 = 777;
pub const NT_S390_VXRS_HIGH: u32 = 778;
pub const NT_S390_GS_CB: u32 = 779;
pub const NT_S390_GS_BC: u32 = 780;
pub const NT_S390_RI_CB: u32 = 781;
pub const NT_ARM_VFP: u32 = 1024;
pub const NT_ARM_TLS: u32 = 1025;
pub const NT_ARM_HW_BREAK: u32 = 1026;
pub const NT_ARM_HW_WATCH: u32 = 1027;
pub const NT_ARM_SYSTEM_CALL: u32 = 1028;
pub const NT_ARM_SVE: u32 = 1029;
pub const NT_ARM_PAC_MASK: u32 = 1030;
pub const NT_ARM_PACA_KEYS: u32 = 1031;
pub const NT_ARM_PACG_KEYS: u32 = 1032;
pub const NT_ARM_TAGGED_ADDR_CTRL: u32 = 1033;
pub const NT_ARM_PAC_ENABLED_KEYS: u32 = 1034;
pub const NT_ARC_V2: u32 = 1536;
pub const NT_VMCOREDD: u32 = 1792;
pub const NT_MIPS_DSP: u32 = 2048;
pub const NT_MIPS_FP_MODE: u32 = 2049;
pub const NT_MIPS_MSA: u32 = 2050;
pub const NT_GNU_PROPERTY_TYPE_0: u32 = 5;
pub const GNU_PROPERTY_AARCH64_FEATURE_1_AND: u32 = 3221225472;
pub const GNU_PROPERTY_AARCH64_FEATURE_1_BTI: u32 = 1;
pub type Elf32_Addr = u32;
pub type Elf32_Half = u16;
pub type Elf32_Off = u32;
pub type Elf32_Word = u32;
pub type Elf64_Addr = u64;
pub type Elf64_Half = u16;
pub type Elf64_Off = u64;
pub type Elf64_Word = u32;
pub type Elf64_Xword = u64;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct elf32_hdr {
    pub e_ident: [::std::os::raw::c_uchar; 16usize],
    pub e_type: Elf32_Half,
    pub e_machine: Elf32_Half,
    pub e_version: Elf32_Word,
    pub e_entry: Elf32_Addr,
    pub e_phoff: Elf32_Off,
    pub e_shoff: Elf32_Off,
    pub e_flags: Elf32_Word,
    pub e_ehsize: Elf32_Half,
    pub e_phentsize: Elf32_Half,
    pub e_phnum: Elf32_Half,
    pub e_shentsize: Elf32_Half,
    pub e_shnum: Elf32_Half,
    pub e_shstrndx: Elf32_Half,
}
pub type Elf32_Ehdr = elf32_hdr;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct elf64_hdr {
    pub e_ident: [::std::os::raw::c_uchar; 16usize],
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub e_shoff: Elf64_Off,
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}
pub type Elf64_Ehdr = elf64_hdr;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct elf32_phdr {
    pub p_type: Elf32_Word,
    pub p_offset: Elf32_Off,
    pub p_vaddr: Elf32_Addr,
    pub p_paddr: Elf32_Addr,
    pub p_filesz: Elf32_Word,
    pub p_memsz: Elf32_Word,
    pub p_flags: Elf32_Word,
    pub p_align: Elf32_Word,
}
pub type Elf32_Phdr = elf32_phdr;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct elf64_phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}
pub type Elf64_Phdr = elf64_phdr;
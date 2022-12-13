// This code was autogenerated with `dbus-codegen-rust -s -m None`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgChromiumVtpm {
    fn send_command(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgChromiumVtpm for blocking::Proxy<'a, C> {

    fn send_command(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.Vtpm", "SendCommand", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }
}
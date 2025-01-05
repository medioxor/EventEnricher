#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]

pub use cty::*;

include!(concat!(env!("OUT_DIR"), "/etw.rs"));

#[link(name = "etw_provider")]
unsafe extern "C" {
    pub fn _EventRegisterEventEnricher();
    pub fn _EventUnregisterEventEnricher();
    pub fn _EventWriteProcessNotify(
        Activity: *mut GUID,
        EventTime: *mut FILETIME,
        FilePath: PCWSTR,
        CommandLine: PCWSTR,
        ProcessId: c_ulonglong,
    );
    pub fn _EventWriteThreadNotify(
        Activity: *mut GUID,
        EventTime: *mut FILETIME,
        ProcessId: c_ulonglong,
        ThreadId: c_ulonglong,
    );
}

pub use self::_EventRegisterEventEnricher as EventRegisterEventEnricher;
pub use self::_EventUnregisterEventEnricher as EventUnregisterEventEnricher;
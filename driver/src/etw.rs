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
        ProcessId: c_ulonglong,
        FilePath: PCWSTR,
        CommandLine: PCWSTR,
    );
    pub fn _EventWriteThreadNotify(
        Activity: *mut GUID,
        EventTime: *mut FILETIME,
        ProcessId: c_ulonglong,
        ThreadId: c_ulonglong,
    );
    pub fn _KeQuerySystemTime(SystemTime: *mut FILETIME);
}

impl FILETIME {
    pub fn as_mut_ptr(&mut self) -> *mut FILETIME {
        self as *mut FILETIME
    }
}

pub fn get_system_time() -> FILETIME {
    let mut time: FILETIME = FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    };

    unsafe {
        KeQuerySystemTime(&mut time);
    }

    time
}

pub use self::_KeQuerySystemTime as KeQuerySystemTime;
pub use self::_EventRegisterEventEnricher as EventRegisterEventEnricher;
pub use self::_EventUnregisterEventEnricher as EventUnregisterEventEnricher;
pub use self::_EventWriteProcessNotify as EventWriteProcessNotify;
pub use self::_EventWriteThreadNotify as EventWriteThreadNotify;
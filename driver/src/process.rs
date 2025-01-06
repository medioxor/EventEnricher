use wdk_sys::{ntddk::{
    PsSetCreateProcessNotifyRoutineEx, PsSetCreateThreadNotifyRoutine, PsRemoveCreateThreadNotifyRoutine,
}, BOOLEAN, HANDLE, PEPROCESS, PS_CREATE_NOTIFY_INFO, NTSTATUS};
use wdk::nt_success;
use core::ptr::null_mut;
use crate::etw::{EventWriteProcessNotify, EventWriteThreadNotify, get_system_time, FILETIME};
use crate::string::UnicodeString;

pub extern "C" fn process_notify(_process: PEPROCESS, process_id: HANDLE, created: *mut PS_CREATE_NOTIFY_INFO) {
    let mut event_time = get_system_time();
    
    let created: Option<&PS_CREATE_NOTIFY_INFO> = if !created.is_null() {
        Some(unsafe { &*created })
    } else {
        None
    };
    
    let image_name = if let Some(created) = created {
        UnicodeString::from(created.ImageFileName)
    } else {
        UnicodeString::default()
    };
    
    let command_line = if let Some(created) = created {
        UnicodeString::from(created.CommandLine)
    } else {
        UnicodeString::default()
    };

    unsafe {
        EventWriteProcessNotify(null_mut(), &mut event_time as *mut FILETIME, image_name.0.Buffer, command_line.0.Buffer, process_id as u64);
    }
}

pub extern "C" fn thread_notify(process_id: HANDLE, thread_id: HANDLE, _created: BOOLEAN) {
    let mut event_time = get_system_time();
    
    unsafe {
        EventWriteThreadNotify(null_mut(), &mut event_time as *mut FILETIME, process_id as u64, thread_id as u64);
    }
}

pub fn register_process_callbacks() -> Result<(), NTSTATUS> {
    let mut result: NTSTATUS;

    unsafe {
        result = PsSetCreateProcessNotifyRoutineEx(Some(process_notify), false.into());
    }

    if !nt_success(result) {
        return Err(result);
    }

    unsafe {
        result = PsSetCreateThreadNotifyRoutine(Some(thread_notify));
    }

    if !nt_success(result) {
        return Err(result);
    }

    Ok(())
}

pub fn unregister_process_callbacks() -> Result<(), NTSTATUS> {
    let mut result: NTSTATUS;

    unsafe {
        result = PsSetCreateProcessNotifyRoutineEx(Some(process_notify), true.into());
    }

    if !nt_success(result) {
        return Err(result);
    }

    unsafe {
        result = PsRemoveCreateThreadNotifyRoutine(Some(thread_notify));
    }

    if !nt_success(result) {
        return Err(result);
    }

    Ok(())
}
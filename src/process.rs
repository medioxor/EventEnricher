use wdk_sys::{ntddk::{
    PsSetCreateProcessNotifyRoutineEx, PsSetCreateThreadNotifyRoutine,
}, BOOLEAN, HANDLE, PEPROCESS, PS_CREATE_NOTIFY_INFO};



pub extern "C" fn process_notify(process: PEPROCESS, process_id: HANDLE, created: *mut PS_CREATE_NOTIFY_INFO) {
    
}

pub extern "C" fn thread_notify(process_id: HANDLE, thread_id: HANDLE, created: BOOLEAN) {

}

pub fn register_process_callbacks() {
    unsafe {
        PsSetCreateProcessNotifyRoutineEx(Some(process_notify), false.into());
        PsSetCreateThreadNotifyRoutine(Some(thread_notify));
    }
}
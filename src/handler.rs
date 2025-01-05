use wdk_sys::{DEVICE_OBJECT, NTSTATUS, PIRP, STATUS_SUCCESS};

pub extern "C" fn handle_ioctl(device: *mut DEVICE_OBJECT, pirp: PIRP) -> NTSTATUS {
    STATUS_SUCCESS
}
pub extern "C" fn handle_create(device: *mut DEVICE_OBJECT, pirp: PIRP) -> NTSTATUS {
    STATUS_SUCCESS
}

pub extern "C" fn handle_close(device: *mut DEVICE_OBJECT, pirp: PIRP) -> NTSTATUS {
    STATUS_SUCCESS
}

pub extern "C" fn handle_read(device: *mut DEVICE_OBJECT, pirp: PIRP) -> NTSTATUS {
    STATUS_SUCCESS
}

pub extern "C" fn handle_write(device: *mut DEVICE_OBJECT, pirp: PIRP) -> NTSTATUS {
    STATUS_SUCCESS
}
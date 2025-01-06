use wdk_sys::{DEVICE_OBJECT, NTSTATUS, PIRP, STATUS_SUCCESS};

pub extern "C" fn handle_ioctl(_device: *mut DEVICE_OBJECT, _pirp: PIRP) -> NTSTATUS {
    STATUS_SUCCESS
}
pub extern "C" fn handle_create(_device: *mut DEVICE_OBJECT, _pirp: PIRP) -> NTSTATUS {
    STATUS_SUCCESS
}

pub extern "C" fn handle_close(_device: *mut DEVICE_OBJECT, _pirp: PIRP) -> NTSTATUS {
    STATUS_SUCCESS
}

pub extern "C" fn handle_read(_device: *mut DEVICE_OBJECT, _pirp: PIRP) -> NTSTATUS {
    STATUS_SUCCESS
}

pub extern "C" fn handle_write(_device: *mut DEVICE_OBJECT, _pirp: PIRP) -> NTSTATUS {
    STATUS_SUCCESS
}
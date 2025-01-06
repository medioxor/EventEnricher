#![no_std]
extern crate alloc;

#[cfg(not(test))]
extern crate wdk_panic;

pub mod etw;
pub use cty::*;

mod string;
mod process;
mod handler;

use core::ptr::null_mut;
use wdk_alloc::WdkAllocator;
use wdk::{nt_success, println};
use wdk_sys::{
    ntddk::{IoCreateDevice, IoCreateSymbolicLink, IoDeleteDevice, IoDeleteSymbolicLink}, DRIVER_OBJECT, FILE_DEVICE_SECURE_OPEN, FILE_DEVICE_UNKNOWN, IRP_MJ_CLOSE, IRP_MJ_CREATE, IRP_MJ_DEVICE_CONTROL, IRP_MJ_READ, IRP_MJ_WRITE, NTSTATUS, PCUNICODE_STRING, PDEVICE_OBJECT, STATUS_SUCCESS, TRUE, UNICODE_STRING
};
use string::UnicodeString;
use handler::{handle_close, handle_create, handle_ioctl, handle_read, handle_write};
use process::{register_process_callbacks, unregister_process_callbacks};
use crate::etw::{EventRegisterEventEnricher, EventUnregisterEventEnricher};

#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;
pub static DEVICE_NAME: &str = "\\Device\\EventEnricher";
pub static SYMLINK_NAME: &str = "\\??\\EventEnricher";

#[unsafe(export_name = "DriverEntry")]
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    println!("[+] EventEnricher: driver loaded");
    
    let mut device: PDEVICE_OBJECT = null_mut();
    let mut status: NTSTATUS;
    let mut device_name: UNICODE_STRING = UnicodeString::from(DEVICE_NAME).into();

    unsafe {
        status = IoCreateDevice(
            driver,
            0,
            &mut device_name,
            FILE_DEVICE_UNKNOWN,
            FILE_DEVICE_SECURE_OPEN,
            TRUE as u8,
            &mut device
        );
    }

    if !nt_success(status) {
        println!("[!] EventEnricher: failed to create device, {:#X}", status as u32);
        return status;
    }
    
    unsafe {
        let mut symlink_name: UNICODE_STRING = UnicodeString::from(SYMLINK_NAME).into();
        status = IoCreateSymbolicLink(
            &mut symlink_name,
            &mut device_name
        );
    }

    if !nt_success(status) {
        println!("[!] EventEnricher: failed to create symbolic link, {status}");
        unsafe { IoDeleteDevice(device) };
        return status;
    }
    
    driver.MajorFunction[IRP_MJ_CREATE as usize] = Some(handle_create);
    driver.MajorFunction[IRP_MJ_CLOSE as usize] = Some(handle_close);
    driver.MajorFunction[IRP_MJ_READ as usize] = Some(handle_read);
    driver.MajorFunction[IRP_MJ_WRITE as usize] = Some(handle_write);
    driver.MajorFunction[IRP_MJ_DEVICE_CONTROL as usize] = Some(handle_ioctl);
    driver.DriverUnload = Some(driver_exit);

    unsafe { EventRegisterEventEnricher() };

    match register_process_callbacks() {
        Ok(_) => println!("[+] EventEnricher: process callbacks registered"),
        Err(e) => println!("[!] EventEnricher: failed to register process callbacks, {:#X}", e as u32)
    }
    
    STATUS_SUCCESS
}

extern "C" fn driver_exit(driver: *mut DRIVER_OBJECT) {
    unsafe {
        let mut symlink_name: UNICODE_STRING = UnicodeString::from(SYMLINK_NAME).into();
        IoDeleteDevice((*driver).DeviceObject);
        let _ = IoDeleteSymbolicLink(&mut symlink_name);
        EventUnregisterEventEnricher();
    };

    match unregister_process_callbacks() {
        Ok(_) => println!("[+] EventEnricher: process callbacks unregistered"),
        Err(e) => println!("[!] EventEnricher: failed to unregister process callbacks, {:#X}", e as u32)
    }

    println!("[+] EventEnricher: driver exit");
}
#![no_std]
extern crate alloc;

use wdk_alloc::WdkAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

use wdk::println;
use wdk_sys::{
    STATUS_SUCCESS,
    DRIVER_OBJECT,
    PCUNICODE_STRING,
    NTSTATUS
};

fn configure_driver(driver: &mut DRIVER_OBJECT, registry_path: PCUNICODE_STRING) {
    
}

#[unsafe(export_name = "DriverEntry")]
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    println!("Hello world!");

    STATUS_SUCCESS
}
use wdk_sys::UNICODE_STRING;
use alloc::{string::String, vec::Vec};
use core::slice;
use core::str;

#[derive(Default)]
pub struct UnicodeString(pub UNICODE_STRING);

impl UnicodeString {
    fn from_str(s: &str) -> Self {
        let mut buffer: Vec<u16> = s.encode_utf16().collect();

        UnicodeString(UNICODE_STRING {
            Length: (buffer.len() * 2) as u16,
            MaximumLength: (buffer.len() * 2) as u16,
            Buffer: buffer.as_mut_ptr(),
        })
    }

    fn to_string(&self) -> String {
        let len = self.0.Length as usize / 2;
        let slice = unsafe { slice::from_raw_parts(self.0.Buffer, len) };
        String::from_utf16_lossy(slice)
    }
}

impl From<&str> for UnicodeString {
    fn from(s: &str) -> Self {
        UnicodeString::from_str(s)
    }
}

impl From<String> for UnicodeString {
    fn from(s: String) -> Self {
        UnicodeString::from_str(&s)
    }
}

impl Into<String> for UnicodeString {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Into<UNICODE_STRING> for UnicodeString {
    fn into(self) -> UNICODE_STRING {
        self.0
    }
}

impl Into<String> for &UnicodeString {
    fn into(self) -> String {
        self.to_string()
    }
}

impl From<*const UNICODE_STRING> for UnicodeString {
    fn from(ptr: *const UNICODE_STRING) -> Self {
        if ptr.is_null() {
            return UnicodeString::default();
        }
        let unicode_string = unsafe { *ptr };
        if unicode_string.Buffer.is_null() {
            return UnicodeString::default();
        }
        UnicodeString(unicode_string)
    }
}

impl From<UNICODE_STRING> for UnicodeString {
    fn from(unicode_string: UNICODE_STRING) -> Self {
        if unicode_string.Buffer.is_null() {
            return UnicodeString::default();
        }
        UnicodeString(unicode_string)
    }
}
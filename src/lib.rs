//use winrt::*;

use std::ffi::{CStr, CString, c_void, OsString};
use libc::{c_char};

// use winrt::windows::storage::pickers::{FileOpenPicker, PickerViewMode, PickerLocationId, IFileOpenPicker, IFileOpenPicker2};

// use winrt::windows::devices::enumeration::*;
// use winrt::windows::devices::midi::*;
// use winrt::windows::foundation::*;
// use winrt::windows::foundation::collections::*;
// use winrt::windows::ui::{core::*,shell::*,  ColorHelper, IColorHelper, };
use winapi::um::{
    commdlg::{
        OPENFILENAMEA, GetOpenFileNameA, GetSaveFileNameA,
    }
};



pub fn open_dialog<T: Sized>(hwnd: *mut T) -> Option<String> {
    let mut open: OPENFILENAMEA = unsafe { std::mem::zeroed() };
    let mut cs: Vec<c_char> = vec![0; 512];
    open.lStructSize = std::mem::size_of::<OPENFILENAMEA>() as _;
    open.hwndOwner = unsafe { std::mem::transmute(hwnd) };
    open.lpstrFile = unsafe { std::mem::transmute(cs.as_mut_ptr()) };
    open.nMaxFile = 512;

    let res = unsafe { GetOpenFileNameA(&mut open) };
    if (res != 0) {
        let s = unsafe { CStr::from_ptr(cs.as_ptr()) };
        Some(s.to_str().unwrap().to_owned())
    } else {
        None
    }
}

pub fn open_dialog_windowless() -> Option<String> {
    let mut open: OPENFILENAMEA = unsafe { std::mem::zeroed() };
    let mut cs: Vec<c_char> = vec![0; 512];
    open.lStructSize = std::mem::size_of::<OPENFILENAMEA>() as _;
    open.lpstrFile = unsafe { std::mem::transmute(cs.as_mut_ptr()) };
    open.nMaxFile = 512;

    let res = unsafe { GetOpenFileNameA(&mut open) };
    if (res != 0) {
        let s = unsafe { CStr::from_ptr(cs.as_ptr()) };
        Some(s.to_str().unwrap().to_owned())
    } else {
        None
    }
}

pub fn save_dialog<T: Sized>(hwnd: *mut T) -> Option<String> {
    let mut open: OPENFILENAMEA = unsafe { std::mem::zeroed() };
    let mut cs: Vec<c_char> = vec![0; 512];
    open.lStructSize = std::mem::size_of::<OPENFILENAMEA>() as _;
    open.hwndOwner = unsafe { std::mem::transmute(hwnd) };
    open.lpstrFile = unsafe { std::mem::transmute(cs.as_mut_ptr()) };
    open.nMaxFile = 512;

    let res = unsafe { GetSaveFileNameA(&mut open) };
    if (res != 0) {
        let s = unsafe { CStr::from_ptr(cs.as_ptr()) };
        Some(s.to_str().unwrap().to_owned())
    } else {
        None
    }
}
pub fn save_dialog_windowless() -> Option<String> {
    let mut open: OPENFILENAMEA = unsafe { std::mem::zeroed() };
    let mut cs: Vec<c_char> = vec![0; 512];
    open.lStructSize = std::mem::size_of::<OPENFILENAMEA>() as _;
    open.lpstrFile = unsafe { std::mem::transmute(cs.as_mut_ptr()) };
    open.nMaxFile = 512;

    let res = unsafe { GetSaveFileNameA(&mut open) };
    if (res != 0) {
        let s = unsafe { CStr::from_ptr(cs.as_ptr()) };
        Some(s.to_str().unwrap().to_owned())
    } else {
        None
    }
}







#[cfg(windows)] extern crate winapi;
use std::io::Error;

#[cfg(windows)]
fn get_system_default_lcid() {
    // GetSystemDefaultLCID
    use winapi::um::winnls::GetSystemDefaultLCID;
    let lcid = unsafe {
        GetSystemDefaultLCID()
    };
    
    match lcid {
        0x804 => print_message("中文简体").unwrap(),
        0x404 => print_message("中文繁体").unwrap(),
        0x409 => print_message("美国英语").unwrap(),
        _ => print_message("其它").unwrap(),
    };    
}

#[cfg(not(windows))]
fn is_system_language_zh() -> bool {
    true
}

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}
#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}

fn main() {
    get_system_default_lcid();
}

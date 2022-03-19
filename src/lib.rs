use std::os::raw::c_char;
use std::ffi::CString;
use std::ffi::CStr;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::{Sender, Receiver, channel, TryRecvError};
use std::time::Duration;
use once_cell::unsync::Lazy;
use parking_lot::const_mutex;
use parking_lot::Mutex;

type GMReal = f64;
type GMStr = *mut c_char;
type GMStrArg = *const i8;

fn gmstr_to_str(arg:GMStrArg) -> &'static str {
    let c_str: &CStr = unsafe { CStr::from_ptr(arg) };
    let str_slice: &str = c_str.to_str().unwrap();
    str_slice
}

fn string_to_gmstr(arg:String) -> GMStr {
    CString::new(arg.as_str()).expect("error").into_raw()
}
fn str_to_gmstr(arg:&str) -> GMStr {
    CString::new(arg).expect("error").into_raw()
}

////////////////////////////////////////////////////////////////////////////////
    
static CHANNEL: Mutex<Lazy<(Sender<DebouncedEvent>, Receiver<DebouncedEvent>)>> = const_mutex(Lazy::new(|| channel()));
    
#[no_mangle]
pub extern fn __gm82live_dll_fw_init(directory: GMStrArg) -> GMReal {
    let directory = gmstr_to_str(directory);
    
    // Create a channel to receive the events.
    let (sender, _) = &**CHANNEL.lock();
    
    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = Box::new(watcher(sender.clone(), Duration::from_secs(1)).unwrap());
    
    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(directory, RecursiveMode::Recursive).unwrap();
    
    // Bestow the watcher with immortality.
    Box::leak(watcher);    
    
    0.0
}

#[no_mangle]
pub extern fn __gm82live_dll_fw_poll() -> GMStr {
    let (_, receiver) = &**CHANNEL.lock();
    match receiver.try_recv() {
        Ok(event) => string_to_gmstr(format!("{:?}", event)),
        Err(TryRecvError::Empty) => str_to_gmstr("no events"),
        Err(_) => str_to_gmstr("disconnected"),
    }
}
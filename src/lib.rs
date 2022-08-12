use std::ffi::CString;
use std::ffi::CStr;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::{Sender, Receiver, channel, TryRecvError};
use std::time::Duration;
use once_cell::unsync::Lazy;
use parking_lot::const_mutex;
use parking_lot::Mutex;

type GMReal = f64;
type GMStr = *const i8;

static mut GMSTRING: Lazy<CString> = Lazy::new(|| CString::new("").expect("error"));

fn gmstr_to_string(arg:GMStr) -> &'static str {
    let c_str: &CStr = unsafe { CStr::from_ptr(arg) };
    let str_slice: &str = c_str.to_str().unwrap();
    str_slice
}

fn string_to_gmstr(arg:impl AsRef<str>) -> GMStr {
    unsafe {
        *GMSTRING = CString::new(arg.as_ref()).expect("error");
        GMSTRING.as_ptr()
    }
}

////////////////////////////////////////////////////////////////////////////////
    
static CHANNEL: Mutex<Lazy<(Sender<DebouncedEvent>, Receiver<DebouncedEvent>)>> = const_mutex(Lazy::new(|| channel()));
    
#[no_mangle]
pub extern fn __gm82live_dll_fw_init(directory: GMStr) -> GMReal {
    let directory = gmstr_to_string(directory);
    
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
        Err(TryRecvError::Empty) => string_to_gmstr("no events"),
        Err(_) => string_to_gmstr("disconnected"),
    }
}
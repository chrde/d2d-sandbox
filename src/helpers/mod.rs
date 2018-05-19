use std::io;
use helpers::wnd::create_wnd;
use gui;

use winapi::um::winuser::MSG;
use helpers::msg::Msg;

use winapi::um::winuser::WM_QUIT;
use winapi::shared::minwindef::BOOL;

pub mod strings;
pub mod wnd;
pub mod msg;
pub mod event;

pub fn start_loop() -> io::Result<i32> {
    let _wnd = create_wnd(Some(gui::wnd_proc));
    loop {
        match MSG::get().unwrap() {
            MSG { message: WM_QUIT, wParam: code, .. } => {
                return Ok(code as i32);
            }
            msg => {
                msg.translate();
                msg.dispatch();
            }
        }
    }
}

pub fn last_error<T>() -> io::Result<T> {
    Err(io::Error::last_os_error())
}


pub fn verify<T>(arg: *mut T) {
    match arg {
        v if v.is_null() => last_error(),
        _ => Ok(())
    }.unwrap();
}

pub fn verify_bool(arg: BOOL) {
    match arg {
        1 => Ok(()),
        _ => last_error(),
    }.unwrap();
}

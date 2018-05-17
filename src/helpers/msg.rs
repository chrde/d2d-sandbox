use winapi::shared::minwindef::{
    LRESULT,
    INT,
};
use winapi::um::winuser::{
    GetMessageW,
    MSG,
    TranslateMessage,
    DispatchMessageW,
    PostQuitMessage,
};
use std::io;
use std::mem;
use std::ptr;
use helpers;

pub trait Msg: Sized {
    fn get() -> io::Result<Self>;
    fn dispatch(&self) -> LRESULT;
    fn translate(&self) -> bool;
    fn post_quit(exit_code: INT);
}

impl Msg for MSG {
    fn get() -> io::Result<Self> {
        unsafe {
            let mut msg = mem::zeroed();
            match GetMessageW(&mut msg, ptr::null_mut(), 0, 0) {
                -1 => helpers::last_error(),
                _ => Ok(msg)
            }
        }
    }
    fn dispatch(&self) -> LRESULT {
        unsafe {
            DispatchMessageW(self)
        }
    }

    fn translate(&self) -> bool {
        unsafe {
            match TranslateMessage(self) {
                0 => false,
                _ => true
            }
        }
    }

    fn post_quit(exit_code: INT) {
        unsafe {
            PostQuitMessage(exit_code)
        }
    }
}
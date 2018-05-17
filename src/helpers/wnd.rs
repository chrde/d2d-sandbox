use std::{ptr, mem};
use winapi::um::winuser::CreateWindowExW;
use winapi::um::winuser::CW_USEDEFAULT;
use winapi::um::winuser::CS_HREDRAW;
use winapi::um::winuser::CS_VREDRAW;
use winapi::um::winuser::RegisterClassExW;
use winapi::shared::windef::HWND;
use helpers::strings::get_string;
use helpers::strings::MAIN_WND_CLASS;
use helpers::strings::MAIN_WND_NAME;
use winapi::um::winuser::WNDCLASSEXW;
use winapi::shared::minwindef::UINT;
use winapi::shared::minwindef::WPARAM;
use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::LRESULT;
use winapi::um::winuser::WS_VISIBLE;
use winapi::um::winuser::WS_OVERLAPPEDWINDOW;

pub type WndProcRef = unsafe extern "system" fn(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT;

pub fn create_wnd(wnd_proc: Option<WndProcRef>) -> HWND{
    create_class(wnd_proc);
    unsafe {
        CreateWindowExW(
            0,
            get_string(MAIN_WND_CLASS),
            get_string(MAIN_WND_NAME),
            WS_VISIBLE | WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        )
    }
}

fn create_class(wnd_proc: Option<WndProcRef>) {
    unsafe {
        let class = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: wnd_proc,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: ptr::null_mut(),
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null_mut(),
            lpszClassName: get_string(MAIN_WND_CLASS),
            hIconSm: ptr::null_mut(),
        };
        RegisterClassExW(&class);
    }
}
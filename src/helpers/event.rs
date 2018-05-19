use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::WPARAM;
use winapi::shared::minwindef::UINT;
use winapi::um::winuser::WM_MOUSEMOVE;
use winapi::shared::windowsx::GET_X_LPARAM;
use winapi::shared::windowsx::GET_Y_LPARAM;
use winapi::shared::windef::HWND;

#[derive(Copy, Clone)]
pub struct Event {
    pub wnd: HWND,
    pub l_param: LPARAM,
    pub w_param: WPARAM,
    pub message: UINT,
}

impl Event {
    pub fn mouse_pos(&self) -> (i32, i32) {
        assert_eq!(self.message, WM_MOUSEMOVE);
        (GET_X_LPARAM(self.l_param), GET_Y_LPARAM(self.l_param))
    }
}
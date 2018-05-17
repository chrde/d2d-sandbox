use winapi::shared::minwindef::LRESULT;
use winapi::shared::minwindef::UINT;
use winapi::shared::minwindef::WPARAM;
use winapi::shared::minwindef::LPARAM;
use winapi::um::winuser::WM_CREATE;
use winapi::um::winuser::SetWindowLongPtrW;
use winapi::um::winuser::GWLP_USERDATA;
use winapi::shared::basetsd::LONG_PTR;
use winapi::um::winuser::GetWindowLongPtrW;
use winapi::shared::windef::HWND;
use winapi::um::winuser::DefWindowProcW;

pub struct Gui {

}

impl Gui {
    pub fn new() -> Gui {
        Gui {}
    }

    fn handle(&mut self, event: Event) -> LRESULT {
        match event.message {
            _ => unsafe { DefWindowProcW(event.wnd, event.message, event.w_param, event.l_param) }
        }
    }
}

pub unsafe extern "system" fn wnd_proc(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if message == WM_CREATE {
        let gui = Box::new(Gui::new());
        SetWindowLongPtrW(wnd, GWLP_USERDATA, Box::into_raw(gui) as LONG_PTR);
        0
    } else {
        let event = Event{wnd, message, l_param, w_param};
        let gui = &mut *(GetWindowLongPtrW(wnd, GWLP_USERDATA) as *mut Gui);
        gui.handle(event)
    }
}

#[derive(Copy, Clone)]
struct Event {
    wnd: HWND,
    l_param: LPARAM,
    w_param: WPARAM,
    message: UINT,
}
use winapi::shared::minwindef::LRESULT;
use winapi::shared::minwindef::UINT;
use winapi::shared::minwindef::WPARAM;
use winapi::shared::minwindef::LPARAM;
use winapi::um::winuser::*;
use winapi::shared::basetsd::LONG_PTR;
use winapi::shared::windef::HWND;
use std::mem;
use direct2d::factory::Factory;
use direct2d::render_target::HwndRenderTarget;
use direct2d::math::SizeU;
use helpers::verify;
use direct2d::RenderTarget;
use winapi::um::d2d1::D2D1_SIZE_U;
use winapi::shared::minwindef::LOWORD;
use winapi::shared::minwindef::HIWORD;
use winapi::shared::windef::RECT;
use winapi::um::d2d1::D2D1_WINDOW_STATE_OCCLUDED;
use direct2d::Error;
use winapi::shared::winerror::D2DERR_RECREATE_TARGET;
use examples::Example;
use helpers::event::Event;
use helpers::wnd::invalidate;
use std::ptr;

pub struct Gui<T: Example> {
    factory: Factory,
    example: T,
    render_target: Option<HwndRenderTarget>,
    state: State,
}

impl <T: Example>Gui<T> {
    pub fn new() -> Gui<T> {
        let factory = Factory::new().unwrap();
        let mut example = T::new();
        example.create_device_independent_resources(&factory);
        let gui = Gui {
            factory,
            example,
            state: State::new(),
            render_target: None,
        };
        gui
    }

    fn handle(&mut self, event: Event) -> LRESULT {
        match event.message {
            WM_PAINT => self.on_paint(event),
            WM_SIZE => self.on_size(event),
            WM_DISPLAYCHANGE => self.on_display_change(event),
            WM_DESTROY => self.on_destroy(event),
            WM_MOUSEMOVE => self.on_mouse_move(event),
            _ => unsafe { DefWindowProcW(event.wnd, event.message, event.w_param, event.l_param) }
        }
    }

    fn on_paint(&mut self, event: Event) -> LRESULT {
        self.state.paint = event;
        let mut ps;
        unsafe {
            ps = mem::uninitialized();
            verify(BeginPaint(event.wnd, &mut ps));
        };
        self.render(event);
        unsafe {
            EndPaint(event.wnd, &ps);
        }
        0
    }

    fn on_size(&mut self, event: Event) -> LRESULT {
        let size = D2D1_SIZE_U { width: LOWORD(event.l_param as u32) as u32, height: HIWORD(event.l_param as u32) as u32 };
        if self.render_target.is_some() {
            if self.render_target.as_mut().unwrap().resize(SizeU(size)).is_err() {
                self.render_target = None;
            }
            invalidate(event.wnd);
        }
        0
    }

    fn on_mouse_move(&mut self, event: Event) -> LRESULT {
        self.state.mouse_pos = event.mouse_pos();
        invalidate(self.state.paint.wnd);
        0
    }

    fn on_display_change(&mut self, event: Event) -> LRESULT {
        invalidate(event.wnd);
        0
    }

    fn on_destroy(&mut self, _event: Event) -> LRESULT {
        unsafe {
            PostQuitMessage(0);
        }
        0
    }

    fn render(&mut self, event: Event) {
        if self.render_target.is_none() {
            let mut rect = unsafe { mem::zeroed::<RECT>() };
            unsafe { GetClientRect(event.wnd, &mut rect) };
            assert_eq!(0, rect.top);
            assert_eq!(0, rect.left);
            let render_target = HwndRenderTarget::create(&self.factory)
                .with_hwnd(event.wnd)
                .with_pixel_size(rect.right as u32, rect.bottom as u32)
                .build().unwrap();
            self.render_target = Some(render_target);
            self.example.create_device_resources(self.render_target.as_ref().unwrap());
        }
        unsafe {
            let state = (*(self.render_target.as_mut().unwrap().get_raw())).CheckWindowState();
            if state != D2D1_WINDOW_STATE_OCCLUDED {
                self.render_target.as_mut().unwrap().begin_draw();
                self.example.draw(&self.state, self.render_target.as_mut().unwrap());
                match self.render_target.as_mut().unwrap().end_draw() {
                    Err((Error::Dxgi(v), _)) if v.0 == D2DERR_RECREATE_TARGET => {
                        self.render_target = None;
                        invalidate(event.wnd);
                    },
                    _ => {},
                };
            }
        }
    }
}

pub unsafe extern "system" fn wnd_proc(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if message == WM_CREATE {
        let gui = Box::new(Gui::<::examples::geometries::Geometries>::new());
        SetWindowLongPtrW(wnd, GWLP_USERDATA, Box::into_raw(gui) as LONG_PTR);
        0
    } else {
        let event = Event { wnd, message, l_param, w_param };
        let gui = &mut *(GetWindowLongPtrW(wnd, GWLP_USERDATA) as *mut Gui<::examples::geometries::Geometries>);
        gui.handle(event)
    }
}

pub struct State {
    pub mouse_pos: (i32, i32),
    pub paint: Event,
}

impl State {
    pub fn new() -> State {
        State {
            mouse_pos: (0, 0),
            paint: Event {wnd: ptr::null_mut(), l_param: 0, w_param: 0, message: 0},
        }
    }
}
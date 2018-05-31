use direct2d::render_target::HwndRenderTarget;
use gui::State;
use direct2d::Factory;

//pub mod brushes;
pub mod geometries;

const COLOR_BLUE: u32 = 0x2E_75_E8;
const COLOR_BLACK: u32 = 0x00_00_00;
const COLOR_WHITE: u32 = 0xFF_FF_FF;
const COLOR_YELLOW: u32 = 0xE5_D3_32;
const COLOR_RED: u32 = 0xFF_7F_7F;
const COLOR_GREEN: u32 = 0x7F_FF_7F;

pub trait Example: Sized {
    fn new() -> Self;
    fn create_device_resources(&mut self, render_target: &HwndRenderTarget);
    fn create_device_independent_resources(&mut self, factory: &Factory);
    fn draw(&mut self, state: &State, render_target: &mut HwndRenderTarget);
}
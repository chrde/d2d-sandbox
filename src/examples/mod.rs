use direct2d::render_target::HwndRenderTarget;
use gui::State;
use direct2d::Factory;

pub mod brushes;

pub trait Example: Sized {
    fn new() -> Self;
    fn create_device_resources(&mut self, render_target: &HwndRenderTarget);
    fn create_device_independent_resources(&mut self, factory: &Factory);
    fn draw(&mut self, state: &State, render_target: &mut HwndRenderTarget);
}
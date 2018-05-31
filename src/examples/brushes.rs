use direct2d::brush::Brush;
use direct2d::brush::LinearGradientBrush;
use direct2d::brush::SolidColorBrush;
use direct2d::math::Point2F;
use direct2d::render_target::HwndRenderTarget;
use direct2d::RenderTarget;
use examples::Example;
use direct2d::brush::RadialGradientBrush;
use direct2d::enums::*;
use gui::State;
use direct2d::stroke_style::StrokeStyle;
use direct2d::Factory;
use examples::COLOR_BLUE;
use examples::COLOR_BLACK;
use examples::COLOR_WHITE;
use examples::COLOR_YELLOW;

#[derive(Default)]
pub struct Brushes {
    solid_brush: Option<SolidBrush>,
    linear_gradient_brush: Option<LinearGradBrush>,
    radial_gradient_brush: Option<RadialGradBrush>,
    style_strokes_brush: Option<BrushStrokes>,
    stroke_style: Option<StrokeStyle>

}

impl Example for Brushes {
    fn new() -> Brushes {
        Brushes::default()
    }

    fn create_device_resources(&mut self, render_target: &HwndRenderTarget) {
        self.solid_brush = Some(SolidBrush::new(render_target));
        self.linear_gradient_brush = Some(LinearGradBrush::new(render_target));
        self.radial_gradient_brush = Some(RadialGradBrush::new(render_target));
        self.style_strokes_brush = Some(BrushStrokes::new(render_target, self.stroke_style.take().unwrap()));
    }

    fn create_device_independent_resources(&mut self, factory: &Factory) {
        let style = StrokeStyle::create(factory)
            .with_line_join(LineJoin::Round)
            .with_dash_cap(CapStyle::Round)
            .with_dash_style(DashStyle::DashDot)
            .build().unwrap();
        self.stroke_style = Some(style);
    }

    fn draw(&mut self, _state: &State, render_target: &mut HwndRenderTarget) {
        //Choose one
//        self.solid_brush.as_mut().unwrap().draw(_state, render_target);
//        self.linear_gradient_brush.as_mut().unwrap().draw(_state, render_target);
//        self.radial_gradient_brush.as_mut().unwrap().draw(_state, render_target);
        self.style_strokes_brush.as_mut().unwrap().draw(_state, render_target);
    }
}

struct SolidBrush {
    inner: SolidColorBrush,
}

impl SolidBrush {
    fn new(render_target: &HwndRenderTarget) -> Self {
        let inner = SolidColorBrush::create(render_target).with_color(COLOR_BLUE).build().unwrap();
        SolidBrush {
            inner
        }
    }

    fn draw(&mut self, _state: &State, render_target: &mut HwndRenderTarget) {
        render_target.clear(COLOR_BLUE);

        self.inner.set_opacity(1.0);
        let size = render_target.get_size();
        self.inner.set_color(&COLOR_BLACK.into());
        let rect = (100.0, 100.0, size.width - 100.0, 200.0);
        render_target.fill_rectangle(rect, &self.inner);

        self.inner.set_color(&COLOR_WHITE.into());
        let rect = (100.0, 300.0, size.width - 100.0, 400.0);
        render_target.fill_rectangle(rect, &self.inner);

        self.inner.set_opacity(0.5);
        self.inner.set_color(&COLOR_YELLOW.into());
        let rect = (150.0, 150.0, size.width - 150.0, 350.0);
        render_target.fill_rectangle(rect, &self.inner);
    }
}

struct LinearGradBrush {
    inner: LinearGradientBrush,
}

impl LinearGradBrush {
    fn new(render_target: &HwndRenderTarget) -> Self {
        let stops = [
            (0.0, COLOR_WHITE.into()).into(),
            (1.0, COLOR_BLUE.into()).into()];
        let inner = LinearGradientBrush::create(render_target).with_extend_mode(ExtendMode ::Mirror).with_stops(&stops).build().unwrap();
        LinearGradBrush {
            inner
        }
    }

    fn draw(&mut self, _state: &State, render_target: &mut HwndRenderTarget) {
        let size = render_target.get_size();

        self.inner.set_end_point(Point2F::new(size.width / 4.0, size.height / 4.0));
        let rect = (0.0, 0.0, size.width, size.height);
        render_target.fill_rectangle(rect, &self.inner);
    }
}



struct RadialGradBrush {
    inner: RadialGradientBrush,
    mouse_pos: (i32, i32),
}

impl RadialGradBrush {
    fn new(render_target: &HwndRenderTarget) -> Self {
        let stops = [
            (0.0, COLOR_WHITE.into()).into(),
            (1.0, COLOR_BLUE.into()).into()];
        let inner = RadialGradientBrush::create(render_target).with_stops(&stops).build().unwrap();
        RadialGradBrush {
            inner,
            mouse_pos: (0, 0),
        }
    }

    fn draw(&mut self, state: &State, render_target: &mut HwndRenderTarget) {
        if state.mouse_pos != self.mouse_pos {
            self.mouse_pos = state.mouse_pos;
            let center = self.inner.get_center();
            self.inner.set_gradient_origin_offset(Point2F::new(self.mouse_pos.0 as f32 - center.x, self.mouse_pos.1 as f32 - center.y));
        }

        let size = render_target.get_size();

        self.inner.set_center(Point2F::new(size.width / 2.0, size.height / 2.0));
        self.inner.set_radius_x(size.width / 2.0);
        self.inner.set_radius_y(size.height / 2.0);
        let rect = (0.0, 0.0, size.width, size.height);
        render_target.fill_rectangle(rect, &self.inner);
    }
}

struct BrushStrokes {
    inner: SolidColorBrush,
    style: StrokeStyle,
}

impl BrushStrokes {
    fn new(render_target: &HwndRenderTarget, style: StrokeStyle) -> Self {
        let inner = SolidColorBrush::create(render_target).with_color(COLOR_BLUE).build().unwrap();
        BrushStrokes {
            inner,
            style,
        }
    }

    fn draw(&mut self, _state: &State, render_target: &mut HwndRenderTarget) {
        render_target.clear(COLOR_WHITE);
        let size = render_target.get_size();

        let rect = (100.0, 100.0, size.width - 100.0, size.height - 100.0);
        render_target.draw_rectangle(rect, &self.inner, 20.0, Some(&self.style));
    }
}
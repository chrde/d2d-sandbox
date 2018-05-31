use direct2d::brush::Brush;
use direct2d::brush::SolidColorBrush;
use direct2d::enums::*;
use direct2d::Factory;
use direct2d::geometry::Ellipse as EllipseGeom;
use direct2d::geometry::Path;
use direct2d::geometry::Rectangle;
use direct2d::geometry::RoundedRectangle;
use direct2d::math::*;
use direct2d::math::RectF;
use direct2d::render_target::HwndRenderTarget;
use direct2d::RenderTarget;
use examples::COLOR_BLACK;
use examples::COLOR_BLUE;
use examples::COLOR_WHITE;
use examples::COLOR_YELLOW;
use examples::Example;
use gui::State;
use winapi::um::d2d1::D2D1_ROUNDED_RECT;
use direct2d::geometry::path::GeometryBuilder;
use examples::COLOR_RED;
use examples::COLOR_GREEN;

#[derive(Default)]
pub struct Geometries {
    shapes: Option<Shapes>,
    simple_geometries: Option<SimpleGeometries>,
    simple_geometries_resources: Option<SimpleGeometriesResources>,
    path_geometries: Option<PathGeometries>,
    path_geometries_resources: Option<PathGeometriesResources>,
    arc_geometries: Option<ArcGeometries>,
    arc_geometries_resources: Option<ArcGeometriesResources>,
    bezier_geometries: Option<BezierGeometries>,
    bezier_geometries_resources: Option<BezierGeometriesResources>,
}

impl Example for Geometries {
    fn new() -> Self {
        Default::default()
    }

    fn create_device_resources(&mut self, render_target: &HwndRenderTarget) {
        self.shapes = Some(Shapes::new(render_target));
        self.simple_geometries = Some(SimpleGeometries::new(render_target, self.simple_geometries_resources.take().unwrap()));
        self.path_geometries = Some(PathGeometries::new(render_target, self.path_geometries_resources.take().unwrap()));
        self.arc_geometries = Some(ArcGeometries::new(render_target, self.arc_geometries_resources.take().unwrap()));
        self.bezier_geometries = Some(BezierGeometries::new(render_target, self.bezier_geometries_resources.take().unwrap()));
    }

    fn create_device_independent_resources(&mut self, factory: &Factory) {
        self.simple_geometries_resources = Some(SimpleGeometries::create_device_independent_resources(factory));
        self.path_geometries_resources = Some(PathGeometries::create_device_independent_resources(factory));
        self.arc_geometries_resources = Some(ArcGeometries::create_device_independent_resources(factory));
        self.bezier_geometries_resources = Some(BezierGeometries::create_device_independent_resources(factory));
    }

    fn draw(&mut self, _state: &State, render_target: &mut HwndRenderTarget) {
//        self.shapes.as_mut().unwrap().draw(_state, render_target);
//        self.simple_geometries.as_mut().unwrap().draw(_state, render_target);
//        self.path_geometries.as_mut().unwrap().draw(_state, render_target);
//        self.arc_geometries.as_mut().unwrap().draw(_state, render_target);
        self.bezier_geometries.as_mut().unwrap().draw(_state, render_target);
    }
}

struct BezierGeometries {
    brush: SolidColorBrush,
    path1: Path,
    path2: Path,
    p1: Point2F,
    p2: Point2F,
}

type BezierGeometriesResources = (Path, Path, Point2F, Point2F);

impl BezierGeometries {
    fn create_device_independent_resources(factory: &Factory) -> (BezierGeometriesResources) {
        let begin = (100.0, 600.0);
        let end = (900.0, 600.0).into();
        let p1 = (50.0, 50.0).into();
        let p2 = (600.0, 50.0).into();
        let mut path1 = Path::create(factory).unwrap();
        {
            let segment = BezierSegment::new(p1, p2, end);
            path1.open().unwrap().begin_figure(begin, FigureBegin::Filled, FigureEnd::Closed).add_bezier(&segment).end();
        }
        let mut path2 = Path::create(factory).unwrap();
        {
            let segments = [QuadBezierSegment::new((400.0, 0.0).into(), (400.0, 300.0).into()), QuadBezierSegment::new((400.0, 600.0).into(), end)];
            path2.open().unwrap().begin_figure(begin, FigureBegin::Filled, FigureEnd::Closed).add_quadratic_beziers(&segments).end();
        }
        (path1, path2, p1, p2)
    }

    fn new(render_target: &HwndRenderTarget, resources: BezierGeometriesResources) -> Self {
        let brush = SolidColorBrush::create(render_target).with_color(COLOR_BLUE).build().unwrap();
        BezierGeometries {
            brush,
            path1: resources.0,
            path2: resources.1,
            p1: resources.2,
            p2: resources.3,
        }
    }

    fn draw(&mut self, _state: &State, render_target: &mut HwndRenderTarget) {
        render_target.clear(COLOR_BLUE);

        self.brush.set_color(&COLOR_WHITE.into());
        let ellipse = Ellipse::new(self.p1, 10.0, 10.0);
        render_target.fill_ellipse(ellipse, &self.brush);
        let ellipse = Ellipse::new(self.p2, 10.0, 10.0);
        render_target.fill_ellipse(ellipse, &self.brush);

        render_target.draw_geometry(&self.path1, &self.brush, 10.0, None);
        self.brush.set_color(&COLOR_RED.into());
        render_target.draw_geometry(&self.path2, &self.brush, 10.0, None);
    }
}

struct ArcGeometries {
    brush: SolidColorBrush,
    arc1: Path,
    arc2: Path,
    arc3: Path,
    arc4: Path,
    begin: Point2F,
    end: Point2F,
}

type ArcGeometriesResources = (Path, Path, Path, Path, Point2F, Point2F);

impl ArcGeometries {
    fn create_device_independent_resources(factory: &Factory) -> (ArcGeometriesResources) {
        let begin = (400.0, 200.0).into();
        let end = (600.0, 500.0).into();
        let path1 = ArcGeometries::build_path(factory, begin, end, SweepDirection::CounterClockwise, ArcSize::Large);
        let path2 = ArcGeometries::build_path(factory, begin, end, SweepDirection::Clockwise, ArcSize::Large);
        let path3 = ArcGeometries::build_path(factory, begin, end, SweepDirection::CounterClockwise, ArcSize::Small);
        let path4 = ArcGeometries::build_path(factory, begin, end, SweepDirection::Clockwise, ArcSize::Small);
        (path1, path2, path3, path4, begin, end)
    }

    fn build_path(factory: &Factory, begin: Point2F, end: Point2F, direction: SweepDirection, size: ArcSize) -> Path {
        let mut path = Path::create(&factory).unwrap();
        {
            let builder = path.open().unwrap();
            let arc = ArcSegment::new(end, (200.0, 200.0).into(), 0.0, direction, size);
            builder.begin_figure(begin, FigureBegin::Filled, FigureEnd::Open).add_arc(&arc).end();
        }
        path
    }

    fn new(render_target: &HwndRenderTarget, resources: ArcGeometriesResources) -> Self {
        let brush = SolidColorBrush::create(render_target).with_color(COLOR_BLUE).build().unwrap();
        ArcGeometries {
            brush,
            arc1: resources.0,
            arc2: resources.1,
            arc3: resources.2,
            arc4: resources.3,
            begin: resources.4,
            end: resources.5,
        }
    }

    fn draw(&mut self, _state: &State, render_target: &mut HwndRenderTarget) {
        render_target.clear(COLOR_WHITE);
        self.brush.set_color(&COLOR_YELLOW.into());
        let ellipse = Ellipse::new(self.begin, 50.0, 50.0);
        render_target.fill_ellipse(ellipse, &self.brush);

        self.brush.set_color(&COLOR_BLUE.into());
        let ellipse = Ellipse::new(self.end, 50.0, 50.0);
        render_target.fill_ellipse(ellipse, &self.brush);
        self.brush.set_color(&COLOR_RED.into());
        render_target.draw_geometry(&self.arc1, &self.brush, 10.0, None);
        render_target.draw_geometry(&self.arc2, &self.brush, 10.0, None);

        self.brush.set_color(&COLOR_GREEN.into());
        render_target.draw_geometry(&self.arc3, &self.brush, 10.0, None);
        render_target.draw_geometry(&self.arc4, &self.brush, 10.0, None);
    }
}

struct PathGeometries {
    brush: SolidColorBrush,
    path: Path,
}

type PathGeometriesResources = (Path);

impl PathGeometries {
    fn create_device_independent_resources(factory: &Factory) -> (PathGeometriesResources) {
        let mut path = Path::create(&factory).unwrap();
        {
            let mut builder = path.open().unwrap();
            builder = builder.begin_figure((50.0, 50.0), FigureBegin::Filled, FigureEnd::Closed)
                .add_line((250.0, 30.0))
                .add_lines(&[(270.0, 100.0).into(), (200.0, 100.0).into()])
                .end();

            builder.begin_figure((500.0, 500.0), FigureBegin::Filled, FigureEnd::Closed)
                .add_lines(&[(500.0, 750.0).into(), (750.0, 750.0).into(), (750.0, 500.0).into()])
                .end();
        }
        (path)
    }

    fn new(render_target: &HwndRenderTarget, resources: PathGeometriesResources) -> Self {
        let brush = SolidColorBrush::create(render_target).with_opacity(0.8).with_color(COLOR_BLUE).build().unwrap();
        PathGeometries {
            path: resources,
            brush,
        }
    }

    fn draw(&mut self, _state: &State, render_target: &mut HwndRenderTarget) {
        render_target.clear(COLOR_BLUE);
        self.brush.set_color(&COLOR_YELLOW.into());
        render_target.fill_geometry(&self.path, &self.brush);
        self.brush.set_color(&COLOR_BLACK.into());
        render_target.draw_geometry(&self.path, &self.brush, 10.0, None);
    }
}

struct SimpleGeometries {
    rect: Rectangle,
    rounded_rect: RoundedRectangle,
    ellipse: EllipseGeom,
    brush: SolidColorBrush,
}

type SimpleGeometriesResources = (Rectangle, RoundedRectangle, EllipseGeom);

impl SimpleGeometries {
    fn create_device_independent_resources(factory: &Factory) -> (SimpleGeometriesResources) {
        let rect = (100.0, 100.0, 600.0, 400.0).into();
        let rectangle = Rectangle::create(factory, &rect).unwrap();
        let rounded = rounded_rect(rect, 40.0, 40.0);
        let rounded_rect = RoundedRectangle::create(factory, &rounded).unwrap();
        let center = (rect.width() / 2.0, rect.height() / 2.0).into();
        let ellipse = Ellipse::new(center, center.x - 50.0, center.y - 50.0);
        let ellipse_geom = EllipseGeom::create::<usize>(factory, &ellipse).unwrap();
        (rectangle, rounded_rect, ellipse_geom)
    }

    fn new(render_target: &HwndRenderTarget, resources: SimpleGeometriesResources) -> Self {
        let brush = SolidColorBrush::create(render_target).with_opacity(0.5).with_color(COLOR_WHITE).build().unwrap();
        SimpleGeometries {
            rect: resources.0,
            rounded_rect: resources.1,
            ellipse: resources.2,
            brush,
        }
    }

    fn draw(&mut self, _state: &State, render_target: &mut HwndRenderTarget) {
        render_target.clear(COLOR_BLUE);
        render_target.draw_geometry(&self.rect, &self.brush, 40.0, None);
        render_target.draw_geometry(&self.rounded_rect, &self.brush, 50.0, None);
        render_target.draw_geometry(&self.ellipse, &self.brush, 50.0, None);
    }
}

struct Shapes {
    brush: SolidColorBrush,
}

impl Shapes {
    fn new(render_target: &HwndRenderTarget) -> Self {
        let brush = SolidColorBrush::create(render_target).with_color(COLOR_BLUE).build().unwrap();
        Shapes {
            brush
        }
    }
    fn draw(&mut self, _state: &State, render_target: &mut HwndRenderTarget) {
        render_target.clear(COLOR_BLACK);
        let size = render_target.get_size();
        let offset = 50.0;
        let rect = (offset, offset, size.width - offset, size.height - offset);
        let rounded = rounded_rect(rect, 200.0, 200.0);
        let center = (size.width / 2.0, size.height / 2.0).into();
        let ellipse = Ellipse::new(center, center.x - offset, center.y - offset);

        self.brush.set_color(&COLOR_BLUE.into());
        self.brush.set_opacity(1.0);
        render_target.fill_rectangle(rect, &self.brush);

        self.brush.set_color(&COLOR_BLACK.into());
        render_target.draw_line((offset, offset), (size.width - offset, size.height - offset), &self.brush, 20.0, None);

        self.brush.set_color(&COLOR_WHITE.into());
        self.brush.set_opacity(0.5);
        render_target.draw_rectangle(rect, &self.brush, 20.0, None);
        render_target.draw_rounded_rectangle(rounded, &self.brush, 40.0, None);

        self.brush.set_color(&COLOR_YELLOW.into());
        render_target.draw_ellipse(ellipse, &self.brush, 40.0, None);
    }
}

fn rounded_rect<R: Into<RectF>>(rect: R, radius_x: f32, radius_y: f32) -> RoundedRect {
    RoundedRect(D2D1_ROUNDED_RECT {
        rect: *rect.into(),
        radiusX: radius_x,
        radiusY: radius_y,
    })
}
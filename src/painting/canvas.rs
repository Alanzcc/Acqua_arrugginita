use crate::Palette;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use crate::painting::shapes::Polygon;
use core::f32::consts::PI;
use sdl2::render::WindowCanvas;

pub fn set_pixel(palette: Palette, canvas: &mut WindowCanvas, width: u32, height: u32, prim_color: Color) {
    canvas.set_draw_color(prim_color);
    canvas.clear();
    let exs = palette.check_points();
    for (k, v) in exs.iter() {
        canvas.set_draw_color(*k);
        for p in v {
            if p.x >= 1 && p.x <= width as i32 {
                if p.y >= 1 && p.x <= height as i32 {
                    canvas.draw_point(*p).expect("Expected to draw pixel");
                }
            }
        }
    }
    canvas.present();
}

// Bresenham
pub fn bresenham(palette: &mut Palette, xi: i32, yi: i32, xf: i32, yf: i32, intensity: Color) {
    let dx = (xf - xi).abs();
    let dy = (yf - yi).abs();
    let mut y = yi;
    let mut p = 2 * dy - dx;

    for x in xi..xf {
        if p > 0 {
            y += 1;
            p += 2 * (dy - dx);
        } else {
            p += 2 * dy;
        }
        palette.paint_point(Point::new(x, y), intensity);
    }
}

pub fn dda(palette: &mut Palette, xi: i32, yi: i32, xf: i32, yf: i32, intensity: Color) {
    let dx = xf - xi;
    let dy = yf - yi;
    let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() };
    let step_x = dx / steps;
    let step_y = dy / steps;
    let mut x = xi;
    let mut y = yi;

    for _i in 0..=steps {
        palette.paint_point(Point::new(x, y), intensity);
        x += step_x;
        y += step_y;
    }
}

// Private function for DDA_AA
fn calculate_colors(intensity: Color, prop: f32) -> (Color, Color) {
    let main_color_intensity = ((1.0 - prop) * 255.0).round() as u8;
    let adjacent_color_intensity = (prop * 255.0).round() as u8;
    let main_color = Color::RGBA(intensity.r, intensity.g, intensity.b, main_color_intensity);
    let adjacent_color = Color::RGBA(intensity.r, intensity.g, intensity.b, adjacent_color_intensity);
    (main_color, adjacent_color)
}


pub fn dda_aa(palette: &mut Palette, xi: i32, yi: i32, xf: i32, yf: i32, intensity: Color) {
    let dx = xf as f32 - xi as f32;
    let dy = yf as f32 - yi as f32;

    let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() };
    let step_x = dx / steps;
    let step_y = dy / steps;

    let mut x = xi as f32;
    let mut y = yi as f32;

    palette.paint_point(Point::new(x as i32, y as i32), intensity);

    for _i in 0..=steps as i32 {
        let prop: f32;
        if step_x.abs() == 1.0 {
            prop = (y - y.floor()).abs();
            let (main_color, adjacent_color) = calculate_colors(intensity, prop);
            palette.paint_point(
                Point::new(x.floor() as i32, y.floor() as i32),
                main_color);
            palette.paint_point(
                Point::new(x.floor() as i32, (y + step_y.signum()).floor() as i32),
                adjacent_color);
        } else {
            prop = (x - x.floor()).abs();
            let (main_color, adjacent_color) = calculate_colors(intensity, prop);
            palette.paint_point(
                Point::new(x.floor() as i32, y.floor() as i32),
                main_color);
            palette.paint_point(
                Point::new((x + step_x.signum()).floor() as i32, y.floor() as i32),
                adjacent_color);
        }
        x += step_x;
        y += step_y;
    }
}

pub fn draw_polygon(palette: &mut Palette, polygon: Polygon, intensity: Color) {
    let pol_len = polygon.len();

    for i in 0..(pol_len - 1) {
        let p1 = polygon.read_vertex(i);
        let p2 = polygon.read_vertex(i + 1);

        dda_aa(palette, p1.x, p1.y, p2.x, p2.y, intensity)
    }
    let p0 = polygon.read_vertex(0);
    let pn = polygon.read_vertex(pol_len - 1);
    dda_aa(palette, p0.x, p0.y, pn.x, pn.y, intensity);
}

// Circle polygon should be empty
pub fn calc_circle(center: Point, r: f32) -> Polygon {
    let mut circle = Polygon::new(vec![]);
    let mut angle: f32 = 0.0;
    while angle < 2.0 * PI {
        circle.add_vertex(
            Point::new((center.x as f32 + (r * angle.cos())).floor() as i32,
                       (center.y as f32 + (r * angle.sin())).floor() as i32));
        angle += 0.05;
    }
    circle
}


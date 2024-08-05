use crate::math::Matrix;
use crate::Palette;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub fn set_pixel(screen: &mut Matrix, mut x: usize, mut y: usize, intensity: Color) {
    let c = screen.get_n_cols();
    let r = screen.get_n_rows();
    if x >= c {
        x = c - 1;
    }
    if y >= r {
        y = r - 1;
    }
    screen.set_data(x, y, intensity);
}

pub fun sdl_set_pixel(canvas: &mut Canvas<Window>, x: i32, y: i32, intensity: u8 ) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(intensity, intensity, intensity));
    canvas.draw_point()
}

// Bresenham
pub fn bresenham(canvas: &mut Palette, xi: i32, yi: i32, xf: i32, yf: i32, intensity: Color) {
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
        canvas.paint_point(Point::new(x, y), intensity);
    }
}

pub fn dda(canvas: &mut Palette, xi: i32, yi: i32, xf: i32, yf: i32, intensity: Color) {
    let dx = xf - xi;
    let dy = yf - yi;
    let steps = if dx > dy { dx } else { dy };
    let step_x = dx / steps;
    let step_y = dy / steps;
    let mut x = xi;
    let mut y = yi;

    for _i in 0..=steps {
        canvas.paint_point(Point::new(x, y), intensity);
        x += step_x;
        y += step_y;
    }
}

<<<<<<< Updated upstream
pub fn dda_aa(canvas: &mut Palette, xi: i32, yi: i32, xf: i32, yf: i32, intensity: Color) {
    let dx = (xf - xi) as f32;
    let dy = (yf - yi) as f32;
    let steps = if dx > dy { dx.abs() } else { dy.abs() };
    let step_x = dx / steps;
    let step_y = dy / steps;
    let mut x = xi as f32;
    let mut y = yi as f32;

    for _i in 0..=steps as i32 {
        let prop = if step_x.abs() == 1.0 {
            (y - y.floor()).abs()
        } else {
            (x - x.floor()).abs()
        };

        let main_color_intensity = ((1.0 - prop) * 255.0) as u8;
        let adjacent_color_intensity = (prop * 255.0) as u8;

        let main_color = Color::RGBA(intensity.r, intensity.g, intensity.b, main_color_intensity);
        let adjacent_color = Color::RGBA(intensity.r, intensity.g, intensity.b, adjacent_color_intensity);
        x += step_x;
        y += step_y;
    }
}
=======
>>>>>>> Stashed changes

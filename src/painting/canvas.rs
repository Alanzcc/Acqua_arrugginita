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

// Bresenham
pub fn bresenham(
    canvas: &mut Palette,
    xi: usize,
    yi: usize,
    xf: usize,
    yf: usize,
    intensity: Color,
) {
    let dx = xf - xi;
    let dy = yf - yi;
    let mut y = yi;
    let mut p = 2 * dy - dx;

    for x in xi..xf {
        if p > 0 {
            y += 1;
            p += 2 * (dy - dx);
        } else {
            p += 2 * dy;
        }
        let point = Point::new(x as i32, y as i32);
        canvas.paint_point(point, intensity);
    }
}

pub fn dda(screen: &mut Matrix, xi: usize, yi: usize, xf: usize, yf: usize, intensity: Color) {
    let dx = xf - xi;
    let dy = yf - yi;
    let steps = if dx > dy { dx } else { dy };
    let step_x = dx / steps;
    let step_y = dy / steps;
    let mut x = xi;
    let mut y = yi;

    for _i in 0..=steps {
        set_pixel(screen, x, y, intensity);
        x += step_x;
        y += step_y;
    }
}

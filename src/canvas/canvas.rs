use sdl2::pixels::Color;

use crate::math::Matrix;

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
    screen: &mut Matrix,
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
            y = y + 1;
            p = p + 2 * (dy - dx);
        } else {
            p = p + 2 * dy;
        }
        set_pixel(screen, x, y, intensity);
    }
}

pub fn dda(screen: &mut Matrix, xi: usize, yi: usize, xf: usize, yf: usize, intensity: Color) {
    let dx = xf - xi;
    let dy = yf - yi;

    let steps: usize;
    if dx > dy {
        steps = dx;
    } else {
        steps = dy;
    }

    let step_x = dx / steps;
    let step_y = dy / steps;
    let mut x = xi;
    let mut y = yi;

    for _i in 0..=steps {
        set_pixel(screen, x, y, intensity);
        x = x + step_x;
        y = y + step_y;
    }
}

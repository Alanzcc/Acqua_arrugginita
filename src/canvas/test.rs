use sdl2::pixels::Color;

use crate::math::Matrix;

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

    f

}

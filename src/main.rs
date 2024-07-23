use std::usize;

use math::matrix::Matrix;

mod math;

fn set_pixel(screen: &mut Matrix, mut x: i32, mut y: i32, intensity: i8) {
    let c: i32 = screen.get_n_cols().try_into().unwrap();
    let r: i32 = screen.get_n_rows().try_into().unwrap();

    if x < 0 {
        x = 0;
    }
    if y < 0 {
        y = 0;
    }
    if x >= c {
        x = c - 1;
    }
    if y >= r {
        y = r - 1;
    }
    screen.set_data(x as usize, y as usize, intensity);
}

// Bresenham
fn bresenham(screen: &mut Matrix, xi: i32, yi: i32, xf: i32, yf: i32, intensity: i8) {
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

fn main() {
    println!("Buon giorno!")
}

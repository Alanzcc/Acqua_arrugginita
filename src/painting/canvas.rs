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

fn calculate_colors(intensity: Color, prop: f32) -> (Color, Color) {
    let main_color_intensity = ((1.0 - prop) * 255.0).round() as u8;
    let adjacent_color_intensity = (prop * 255.0).round() as u8;
    let main_color = Color::RGBA(intensity.r, intensity.g, intensity.b, main_color_intensity);
    let adjacent_color = Color::RGBA(intensity.r, intensity.g, intensity.b, adjacent_color_intensity);
    (main_color, adjacent_color)
}


pub fn dda_aa(canvas: &mut Palette, xi: i32, yi: i32, xf: i32, yf: i32, intensity: Color) {
    let dx = (xf - xi) as f32;
    let dy = (yf - yi) as f32;
    let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() };
    let step_x = dx / steps;
    let step_y = dy / steps;
    let mut x = xi as f32;
    let mut y = yi as f32;

    canvas.paint_point(Point::new(x as i32, y as i32), intensity);

    for _i in 0..=steps as i32 {

        x += step_x;
        y += step_y;
        let prop:f32;

        if step_x.abs() == 1.0 {
            prop = (y - y.floor()).abs();
            let (main_color, adjacent_color) = calculate_colors(intensity, prop);
            canvas.paint_point(Point::new(x.floor() as i32, y.floor() as i32), main_color);
            canvas.paint_point(Point::new(x.floor() as i32, (y + step_y.signum()).floor() as i32), adjacent_color);
        } else {
            prop = (x - x.floor()).abs();
            let (main_color, adjacent_color) = calculate_colors(intensity, prop);
            canvas.paint_point(Point::new(x.floor() as i32, y.floor() as i32), main_color);
            canvas.paint_point(Point::new((x + step_x.signum()).floor() as i32, y.floor() as i32), adjacent_color);
        }
        
    }
}

// recebe canvas, polígono, intensidade da cor
// 1. encontrar a menor e a maior coordenada do polígono
// 2. iterar por cada linha horizontal entre yi e yf
// 3. encontra interseções: recebe uma coordenada y, um ponto inicial e um ponto final; retorna a coordenada da interseção ou -1 
// 4. voltar para o primeiro ponto
// 5. preencher a linha atual da imagem entre as interseções encontradas

//3
fn find_intersection(y: f64, pi: Point, pf: Point) -> Option<(Point, Point)> {
    if pi.1 == pf.1 {
        None
    }

    let t = (y - pi.1) / (pf.1 - pi.1);
    if (t < 0.0 || t > 1.0) {
        None
    }

    let x = pi.0 + t * (pf.0 - pi.0);

    Some((x, y))
}

fn print_scan()
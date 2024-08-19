use crate::math::Matrix;
use crate::painting::shapes::Polygon;
use crate::Palette;
use core::f32::consts::PI;
use sdl2::pixels::Color;
use sdl2::rect::Point;
//use sdl2::render::Texture;
//use crate::painting::shapes::Polygon;
//use core::f32::consts::PI;

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
    let steps = if dx > dy { dx } else { dy };
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
    let adjacent_color = Color::RGBA(
        intensity.r,
        intensity.g,
        intensity.b,
        adjacent_color_intensity,
    );
    (main_color, adjacent_color)
}

pub fn dda_aa(palette: &mut Palette, xi: i32, yi: i32, xf: i32, yf: i32, intensity: Color) {
    let dx = (xf - xi) as f32;
    let dy = (yf - yi) as f32;
    let steps = if dx.abs() > dy.abs() {
        dx.abs()
    } else {
        dy.abs()
    };
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
            palette.paint_point(Point::new(x.floor() as i32, y.floor() as i32), main_color);
            palette.paint_point(
                Point::new(x.floor() as i32, (y + step_y.signum()).floor() as i32),
                adjacent_color,
            );
        } else {
            prop = (x - x.floor()).abs();
            let (main_color, adjacent_color) = calculate_colors(intensity, prop);
            palette.paint_point(Point::new(x.floor() as i32, y.floor() as i32), main_color);
            palette.paint_point(
                Point::new((x + step_x.signum()).floor() as i32, y.floor() as i32),
                adjacent_color,
            );
        }
        x += step_x;
        y += step_y;
    }
}

// recebe canvas, polígono, intensidade da cor
// 1. encontrar a menor e a maior coordenada do polígono
// 2. iterar por cada linha horizontal entre yi e yf
// 3. encontra interseções: recebe uma coordenada y, um ponto inicial e um ponto final; retorna a coordenada da interseção ou Nada?
// 4. voltar para o primeiro ponto
// 5. preencher a linha atual da imagem entre as interseções encontradas

//3
fn find_intersection(y: i32, pi: &Point, pf: &Point) -> Option<Point> {
    let mut p_i = pi;
    let mut p_f = pf;

    if p_i.x > p_f.x {
        let aux = p_i;
        p_i = p_f;
        p_f = aux;
    }

    if p_i.y == p_f.y {
        if p_i.y == y {
            return Some(Point::new(p_i.x, y));
        } else {
            return None;
        }
    }

    let t = (y - p_i.y) as f32 / (p_f.y - p_i.y) as f32;
    if t <= 0.0 || t > 1.0 {
        return None;
    }

    let x = p_i.x as f32 + t * (p_f.x - p_i.x) as f32;

    Some(Point::new(x.round() as i32, y))
}

//5

fn print_scan(palette: &mut Palette, p_int: &[Point], intensity: Color) {
    if p_int.len() < 2 {
        //menos de dois pontos de interseção (não desenha nada)
        return;
    }
    //esquerda p/ direita
    for i in (0..p_int.len() - 1).step_by(2) {
        let (x1, x2) = if p_int[i].x > p_int[i + 1].x {
            (p_int[i + 1].x, p_int[i].x)
        } else {
            (p_int[i].x, p_int[i + 1].x)
        };

        for x in x1..=x2 {
            palette.paint_point(Point::new(x, p_int[i].y), intensity);
        }
    }
}


pub fn scanline(palette: &mut Palette, polygon: &Polygon, intensity: Color) {
    let mut yi = i32::MAX;
    let mut yf = i32::MIN;
    //1
    //encontrar o y minimo e maximo do poligono
    for i in 0..polygon.len() {
        let vertex = polygon.read_vertex(i);
        if vertex.y < yi {
            yi = vertex.y;
        }
        if vertex.y > yf {
            yf = vertex.y
        }
    }

    //
    for y in yi..=yf {
        let mut intersections: Vec<Point> = Vec::new();

        let mut previous_vertex = polygon.read_vertex(polygon.len() - 1);
        // D->A, A->B, B->C, C->D //4

        for i in 0..polygon.len() {
            let current_vertex = polygon.read_vertex(i);
            if let Some(intersection) = find_intersection(y, &previous_vertex, &current_vertex) {
                intersections.push(intersection);
            }
            previous_vertex = current_vertex;
        }

        //
        intersections.sort_by(|a, b| a.x.cmp(&b.x));

        print_scan(palette, &intersections, intensity);
    }
}


//Textura
/*pub fn get_texel(texture: &mut Texture, tx: f32, ty: f32) -> Color {
    let query = texture.query();
    let width = query.width;
    let height = query.height;
    let t_x = tx.abs() % 1.0;
    let t_y = ty.abs() % 1.0;

    let x = ((width - 1) as f32 * t_x).round() as u32;
    let y = ((height - 1) as f32 * t_y).round() as u32;


}
    */

//print_scan com textura

/*
fn print_scan(
    palette: &mut Palette,
    p_int: &[Point],
    intensity: Option<Color>,
    mut texture: Option<&mut Texture>,
) {
    if p_int.len() < 2 {
        //menos de dois pontos de interseção (não desenha nada)
        return;
    }
    //esquerda p/ direita
    for i in (0..p_int.len() - 1).step_by(2) {
        let (x1, x2) = if p_int[i].x > p_int[i + 1].x {
            (p_int[i + 1].x, p_int[i].x)
        } else {
            (p_int[i].x, p_int[i + 1].x)
        };

        for x in x1..=x2 {
            let color = if let Some(intensity) = intensity {
                intensity
            } else if let Some(mut texture) = texture.as_mut() {
                let t = (x - x1) as f32 / (x2 - x1) as f32;
                let tex_x = p_int[i].x as f32 + t * (p_int[i + 1].x as f32 - p_int[i].x as f32);
                let tex_y = p_int[i].y as f32 + t * (p_int[i + 1].y as f32 - p_int[i].y as f32);
                get_texel(&mut texture, tex_x, tex_y)
            } else {
                Color::BLACK
            };

            palette.paint_point(Point::new(x, p_int[i].y), color);
        }
    }
} 
*/

//scanline com textura
/*pub fn scanline(
    palette: &mut Palette,
    polygon: &Polygon,
    intensity: Option<Color>,
    texture: Option<&mut Texture>,
) {
    let mut yi = i32::MAX;
    let mut yf = i32::MIN;
    //1
    //encontrar o y minimo e maximo do poligono
    for i in 0..polygon.len() {
        let vertex = polygon.read_vertex(i);
        if vertex.y < yi {
            yi = vertex.y;
        }
        if vertex.y > yf {
            yf = vertex.y
        }
    }

    //
    for y in yi..=yf {
        let mut intersections: Vec<Point> = Vec::new();

        let mut previous_vertex = polygon.read_vertex(polygon.len() - 1);
        // D->A, A->B, B->C, C->D //4

        for i in 0..polygon.len() {
            let current_vertex = polygon.read_vertex(i);
            if let Some(intersection) = find_intersection(y, &previous_vertex, &current_vertex) {
                intersections.push(intersection);
            }
            previous_vertex = current_vertex;
        }

        //
        intersections.sort_by(|a, b| a.x.cmp(&b.x));

        print_scan(palette, &intersections, intensity, texture);
    }
}
*/
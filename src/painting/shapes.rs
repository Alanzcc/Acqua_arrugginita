use std::f32::consts::PI;
use sdl2::rect::Point;

pub struct Polygon {
    vertices: Vec<Point>,
}

impl Polygon {
    pub fn new(vertices: Vec<Point>) -> Self {
        Polygon { vertices }
    }

    pub fn add_vertex(&mut self, point: Point) { self.vertices.push(point); }

    pub fn read_vertex(&self, i: usize) -> Point { self.vertices[i] }

    pub fn len(&self) -> usize { self.vertices.len() }

    pub fn scale(&mut self, k: f32) {
        for i in 0..self.len() {
            self.vertices[i] = Point::new((self.vertices[i].x as f32 * k) as i32, (self.vertices[i].y as f32 * k) as i32);
        }
    }

    pub fn translate(&mut self, k: Point) {
        for i in 0..self.len() {
            self.vertices[i] += k;
        }
    }

    pub fn rotate(&mut self, t: f32) {
        let angle = t * PI / 180.0;
        for i in 0..self.len() {
            let v = self.vertices[i];
            self.vertices[i] = Point::new(
                ((v.x as f32) * angle.cos() - (v.y as f32) * angle.sin()) as i32,
                ((v.x as f32) * angle.sin() + (v.y as f32) * angle.cos()) as i32);
        }
    }

    pub fn stretch_x(&mut self, k: f32) {
        for i in 0..self.len() {
            let v = self.vertices[i];
            self.vertices[i] = Point::new((v.x as f32 * k) as i32, v.y);
        }
    }

    pub fn stretch_y(&mut self, k: f32) {
        for i in 0..self.len() {
            let v = self.vertices[i];
            self.vertices[i] = Point::new(v.x, (v.y as f32 * k) as i32);
        }
    }

    pub fn squeeze_x(&mut self, k: f32) {
        for i in 0..self.len() {
            let v = self.vertices[i];
            self.vertices[i] = Point::new((v.x as f32 * k) as i32, (v.y as f32 / k) as i32);
        }
    }

    pub fn squeeze_y(&mut self, k: f32) {
        for i in 0..self.len() {
            let v = self.vertices[i];
            self.vertices[i] = Point::new((v.x as f32 / k) as i32, (v.y as f32 * k) as i32);
        }
    }

    pub fn shear_x(&mut self, k: f32) {
        for i in 0..self.len() {
            let v = self.vertices[i];
            self.vertices[i] = Point::new((v.x as f32 + v.y as f32 * k) as i32, v.y);
        }
    }

    pub fn shear_y(&mut self, k: f32) {
        for i in 0..self.len() {
            let v = self.vertices[i];
            self.vertices[i] = Point::new(v.x, (v.y as f32 + v.x as f32 * k) as i32);
        }
    }

    fn length(v: &Point) -> f32 { ((v.x * v.x + v.y + v.y) as f32).sqrt() }

    pub fn reflection(&mut self) {
        for i in 0..self.len() {
            let v = self.vertices[i];
            self.vertices[i] = Point::new(v.y, v.x);
        }
    }
}
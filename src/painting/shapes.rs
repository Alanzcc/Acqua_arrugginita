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

    pub fn stretch_x(&self, k: i32) -> Polygon {
        let mut new_pol = Polygon::new(vec![]);
        for v in &self.vertices {
            new_pol.add_vertex(Point::new(v.x * k, v.y));
        }
        new_pol
    }

    pub fn stretch_y(&self, k: i32) -> Polygon {
        let mut new_pol = Polygon::new(vec![]);
        for v in &self.vertices {
            new_pol.add_vertex(Point::new(v.x, v.y * k));
        }
        new_pol
    }

    pub fn squeeze(&self, k: i32) -> Polygon {
        let mut new_pol = Polygon::new(vec![]);
        for v in &self.vertices {
            new_pol.add_vertex(Point::new(v.x * k, v.y / k));
        }
        new_pol
    }

    pub fn rotate(&self, t: f32) -> Polygon {
        let mut new_pol = Polygon::new(vec![]);
        for v in &self.vertices {
            new_pol.add_vertex(Point::new(
                ((v.x as f32) * t.cos() - (v.y as f32) * t.sin()) as i32,
                ((v.x as f32) * t.sin() + (v.y as f32) * t.cos()) as i32));
        }
        new_pol
    }
    pub fn shear_x(&self, k: i32) -> Polygon {
        let mut new_pol = Polygon::new(vec![]);
        for v in &self.vertices {
            new_pol.add_vertex(Point::new(v.x + v.y * k, v.y));
        }
        new_pol
    }

    pub fn shear_y(&self, k: i32) -> Polygon {
        let mut new_pol = Polygon::new(vec![]);
        for v in &self.vertices {
            new_pol.add_vertex(Point::new(v.x, v.y + v.x * k));
        }
        new_pol
    }

    pub fn scale(&self, k: i32) -> Polygon {
        let mut new_pol = Polygon::new(vec![]);
        for v in &self.vertices {
            new_pol.add_vertex(Point::new(v.x * k, v.y * k));
        }
        new_pol
    }

    fn length(v: Point) -> i32 { (v.x * v.x + v.y + v.y) as f32.sqrt() as i32 }

    fn scale_reflect(lv: Vec<i32>, k: i32) -> Vec<i32> {}
    fn reflect(l: Point) {
        let ll = Self::length(l);
        let lv = vec![
            l.x * l.x - l.y * l.y, 2 * l.x * l.y
            2 * l.x * l.y, l.y * l.y + l.x * l.x];
    }
    pub fn reflection(&self) -> Polygon {
        let mut new_pol = Polygon::new(vec![]);
        for v in &self.vertices {}
    }
}

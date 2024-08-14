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

    fn length(v: &Point) -> f32 { ((v.x * v.x + v.y + v.y) as f32).sqrt() }
    fn reflect_vec(l: &Point) -> Vec<i32> {
        vec![
            (l.x * l.x - l.y * l.y) as i32, (2 * l.x * l.y) as i32,
            (2 * l.x * l.y)  as i32, (l.y * l.y + l.x * l.x)  as i32]
    }
    fn scale_reflect(lv: Vec<i32>, k: f32) -> Vec<i32> {
        let s = 1.0 / (k * k);
        vec![(lv[0] as f32 * s) as i32, (lv[1] as f32 * s) as i32,
             (lv[2] as f32 * s) as i32, (lv[3] as f32 * s) as i32]
    }

    pub fn reflection(&self, l: &Point) -> Polygon {
        let mut reflected = Polygon::new(vec![]);
        for v in &self.vertices {
            let lvs = Self::scale_reflect(Self::reflect_vec(&l), Self::length(&l));
            reflected.add_vertex(Point::new(
                lvs[0] * v.x + lvs[1] * v.y,
                lvs[2] * v.x + lvs[3] * v.y));
        }
        reflected
    }
}

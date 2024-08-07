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

}

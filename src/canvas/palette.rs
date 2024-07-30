use sdl2::{pixels::Color, rect::Point};
use std::collections::HashMap;

pub struct Palette {
    color_map: HashMap<Color, Vec<Point>>,
}

impl Palette {
    pub fn new(color_map: HashMap<Color, Vec<Point>>) -> Self {
        Palette { color_map }
    }

    pub fn get_color_map(&self) -> HashMap<Color, Vec<Point>> {
        self.color_map.clone()
    }

    pub fn paint_point(&mut self, point: Point, color: Color) {
        self.color_map.entry(color).or_default().push(point);
    }
}

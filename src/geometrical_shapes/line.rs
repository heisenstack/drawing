use rand::Rang;
use raster::{Color, Image};

pub struct Line{
    a: Point,
    b: Point
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self{
        Self{a: *start, b: *end}
    }
    pub fn random(width: i32, height: i32) -> Self{
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);
        Self::new(&p1, &p2)
    }
}
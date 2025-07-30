use super::point::Point;
use super::line::*;
use crate::geometrical_shapes::{Drawable};
// use raster::{Color, Image};


pub struct Triangle{
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle{
    pub fn new(a: &Point, b:&Point, c:&Point) -> Self{
        Self{
            a: *a,
            b: *b,
            c: *c
        }
    }
}

impl Drawable for Triangle{
    fn draw(&self, img: &mut raster::Image){
        draw_line(img, &self.a, &self.b, self.color());
        draw_line(img, &self.a, &self.c, self.color());
        draw_line(img, &self.b, &self.c, self.color());
    }
}
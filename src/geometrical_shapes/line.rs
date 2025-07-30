use super::point::Point;
use raster::{Color, Image};
use crate::geometrical_shapes::{Displayable, Drawable};

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

impl Drawable for Line{
    fn draw(&self, img: &mut raster::Image) {
        draw_line(img, &self.a, &self.b, self.color())
    }
}

// Bresenham's Line Drawing Algorithm
pub fn draw_line(image: &mut Image, a: &Point, b: &Point, color: Color) {
    let x0 = a.x;
    let y0 = a.y;
    let x1 = b.x;
    let y1 = b.y;

    let dx = (x1 - x0).abs(); // d short for delta
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 }; // s short for step
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut parametre_de_decision = dx + dy; //m 
    let mut x = x0;
    let mut y = y0;

    loop {
        image.display(x, y, color.clone());
        if x == x1 && y == y1 {
            break;
        }
        let p = 2 * parametre_de_decision;
        if p >= dy {
            parametre_de_decision += dy;
            x += sx;
        }
        if p <= dx {
            parametre_de_decision += dx;
            y += sy;
        }
    }
}
use super::line::*;
use super::point::Point;
use crate::geometrical_shapes::Drawable;

// Rectangle: a new rectangle should be created
// from references to two different points.
pub struct Rectangle {
    pub a: Point,
    pub b: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Rectangle { a: *p1, b: *p2 }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut raster::Image) {
        let color = self.color();
        let point_c = &Point::new(self.a.x, self.b.y);
        let point_d = &Point::new(self.b.x, self.a.y);

        draw_line(image, &self.a, point_c, color.clone());
        draw_line(image, &self.a, point_d, color.clone());
        draw_line(image, point_c, &self.b, color.clone());
        draw_line(image, point_d, &self.b, color.clone());
    }
}

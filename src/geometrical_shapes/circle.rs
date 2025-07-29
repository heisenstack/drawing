use super::point::Point;
use crate::geometrical_shapes::{Displayable, Drawable};
use rand::Rng;

pub struct Circle {
    c: Point,
    r: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Circle {
            c: *center,
            r: radius,
        }
    }
    pub fn random(width: i32, height: i32) -> Self {
        let r = rand::rng().random_range(0..width / 2);
        let c = Point::random(width, height);
        Circle::new(&c, r)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut raster::Image) {
        // put pixel
        let cx = self.c.x;
        let cy = self.c.y;

        let mut x = 0;
        let mut y = -self.r;
        let mut p = -self.r;

        let color = self.color();
        while x < -y {
            if p > 0 {
                y += 1;
                p += 2 * (x + y) + 1;
            } else {
                p += 2 * x + 1;
            }
            // putting the pixels
            image.display(cx + x, cy + y, color.clone());
            image.display(cx - x, cy + y, color.clone());
            image.display(cx + x, cy - y, color.clone());
            image.display(cx - x, cy - y, color.clone());

            image.display(cx + y, cy + x, color.clone());
            image.display(cx + y, cy - x, color.clone());
            image.display(cx - y, cy + x, color.clone());
            image.display(cx - y, cy - x, color.clone());

            x += 1;
        }
    }
}

use crate::geometrical_shapes::{Displayable, Drawable};
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn random(width: i32, height: i32) -> Self {
        let mut rnd = rand::rng();
        Point {
            x: rnd.random_range(0..=width),
            y: rnd.random_range(0..=height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut raster::Image) {
        image.display(self.x, self.y, self.color());
    }
}

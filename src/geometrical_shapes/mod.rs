use raster::{Color, Image};
use rand::Rang;

pub trait Displayable{
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable{
    fn draw(&self, img: &mut Image);
    fn color(&self) -> raster::Color{
        let mut rng = rand::rng();
        let color1 = rng.random_range(0..255);
        let color2 = rng.random_range(0..255);
        let color3 = rng.random_range(0..255);
        Color::rgb(color1, color2, color3)
    }
}

pub mod circle;
pub mod line;
pub mod point;
pub mod rectangle;
pub mod triangle;

pub use circle::Circle;
pub use line::Line;
pub use point::Point;
pub use rectangle::Rectangle;
pub use triangle::Triangle;


use rand::Rng;
use raster::Color;

pub trait Displayable {
    fn display(&mut self, coord_x: i32, coord_y: i32, pixel_color: Color);
}

pub trait Drawable {
    fn draw(&self, canvas: &mut impl Displayable);

    fn color(&self) -> Color {
        let red_val = rand::rng().random_range(0..=255);
        let green_val = rand::rng().random_range(0..=255);
        let blue_val = rand::rng().random_range(0..=255);

        Color { r: red_val, g: green_val, b: blue_val, a: 255 }
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(pos_x: i32, pos_y: i32) -> Self {
        Self { x: pos_x, y: pos_y }
    }

    pub fn random(canvas_width: i32, canvas_height: i32) -> Self {
        let rand_x = rand::rng().random_range(1..canvas_width);
        let rand_y = rand::rng().random_range(1..canvas_height);

        Self::new(rand_x, rand_y)
    }
}

impl Drawable for Point {
    fn draw(&self, canvas: &mut impl Displayable) {
        canvas.display(self.x, self.y, self.color());
    }
}

pub struct Circle {
    center_point: Point,
    circle_radius: i32,
}

impl Circle {
    pub fn new(center_pos: Point, radius_val: i32) -> Self {
        Self { center_point: center_pos, circle_radius: radius_val }
    }

    pub fn random(canvas_width: i32, canvas_height: i32) -> Self {
        let random_point = Point::random(canvas_width, canvas_height);
        let random_radius = rand::rng().random_range(0..canvas_width.min(canvas_height) / 2);
        Self::new(random_point, random_radius)
    }
}

impl Drawable for Circle {
    fn draw(&self, canvas: &mut impl Displayable) {
        let center_x = self.center_point.x;
        let center_y = self.center_point.y;
        let radius = self.circle_radius;

        let mut offset_x = 0;
        let mut offset_y = -radius;
        let draw_color = self.color();

        while offset_x < -offset_y {
            let midpoint_y = (offset_y as f32) + 0.5;

            if (offset_x.pow(2) as f32) + midpoint_y.powf(2.0) > (radius.pow(2) as f32) {
                offset_y += 1;
            }

            canvas.display(center_x + offset_x, center_y + offset_y, draw_color.clone()); // top right
            canvas.display(center_x - offset_x, center_y + offset_y, draw_color.clone()); // top left
            canvas.display(center_x - offset_x, center_y - offset_y, draw_color.clone()); // bottom left
            canvas.display(center_x + offset_x, center_y - offset_y, draw_color.clone()); // bottom right

            canvas.display(center_x - offset_y, center_y - offset_x, draw_color.clone()); // right top
            canvas.display(center_x - offset_y, center_y + offset_x, draw_color.clone()); // right bottom
            canvas.display(center_x + offset_y, center_y - offset_x, draw_color.clone()); // left top
            canvas.display(center_x + offset_y, center_y + offset_x, draw_color.clone()); // letf bottom

            offset_x += 1;
        }
    }
}

pub struct Line {
    start_point: Point,
    end_point: Point,
    line_color: Color,
}

impl Line {
    pub fn new(begin: Point, finish: Point, stroke_color: Color) -> Self {
        Self { start_point: begin, end_point: finish, line_color: stroke_color }
    }

    pub fn random(canvas_width: i32, canvas_height: i32) -> Self {
        let first_point = Point::random(canvas_width, canvas_height);
        let second_point = Point::random(canvas_width, canvas_height);

        Self::new(first_point, second_point, first_point.color())
    }
}

impl Drawable for Line {
    fn draw(&self, canvas: &mut impl Displayable) {
        let start_x = self.start_point.x;
        let start_y = self.start_point.y;
        let end_x = self.end_point.x;
        let end_y = self.end_point.y;

        let delta_x = (end_x - start_x) as f32;
        let delta_y = (end_y - start_y) as f32;

        let total_steps = delta_x.abs().max(delta_y.abs()) as i32;

        let step_x = delta_x / (total_steps as f32);
        let step_y = delta_y / (total_steps as f32);

        let mut current_x = start_x as f32;
        let mut current_y = start_y as f32;

        for _ in 0..=total_steps {
            canvas.display(
                current_x.round() as i32,
                current_y.round() as i32,
                self.line_color.clone()
            );
            current_x += step_x;
            current_y += step_y;
        }
    }
}

pub struct Pentagon {
    edge_lines: Vec<Line>,
}

impl Pentagon {
    pub fn new(origin: &Point, edge_length: i32) -> Self {
        let mut current_vertex = *origin;
        let mut rotation_angle: f32 = 0.0;
        let mut pentagon_shape = Pentagon { edge_lines: vec![] };
        let shape_color = origin.color();

        for _ in 1..=5 {
            let horizontal_offset = ((edge_length as f32) *
                rotation_angle.to_radians().cos()) as i32;
            let vertical_offset = ((edge_length as f32) * rotation_angle.to_radians().sin()) as i32;

            let next_vertex = Point::new(
                current_vertex.x + horizontal_offset,
                current_vertex.y + vertical_offset
            );

            pentagon_shape.edge_lines.push(
                Line::new(current_vertex, next_vertex, shape_color.clone())
            );
            current_vertex = next_vertex;

            rotation_angle += 72.0;
        }
        return pentagon_shape;
    }
}

impl Drawable for Pentagon {
    fn draw(&self, canvas: &mut impl Displayable) {
        for edge in &self.edge_lines {
            edge.draw(canvas);
        }
    }
}

pub struct Triangle {
    vertex_a: Point,
    vertex_b: Point,
    vertex_c: Point,
}

impl Triangle {
    pub fn new(first_vertex: &Point, second_vertex: &Point, third_vertex: &Point) -> Self {
        Self {
            vertex_a: *first_vertex,
            vertex_b: *second_vertex,
            vertex_c: *third_vertex,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, canvas: &mut impl Displayable) {
        // let triangle_color = self.color();
        Line::new(self.vertex_a, self.vertex_c, Color::rgb(255, 255, 255)).draw(canvas);
        Line::new(self.vertex_c, self.vertex_b, Color::rgb(255, 255, 255)).draw(canvas);
        Line::new(self.vertex_b, self.vertex_a, Color::rgb(255, 255, 255)).draw(canvas);
    }
}

pub struct Rectangle {
    corner_a: Point,
    corner_b: Point,
    corner_c: Point,
    corner_d: Point,
    rect_color: Color,
}

impl Rectangle {
    pub fn new(first_corner: &Point, second_corner: &Point) -> Self {

        Self {
            corner_a: *first_corner,
            corner_b: *second_corner,
            corner_c: Point { x: first_corner.x, y: second_corner.y },
            corner_d: Point { x: second_corner.x, y: first_corner.y },
            rect_color: Color::rgb(255, 255, 255),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, canvas: &mut impl Displayable) {
        Line::new(self.corner_a, self.corner_c, self.rect_color.clone()).draw(canvas);
        Line::new(self.corner_c, self.corner_b, self.rect_color.clone()).draw(canvas);
        Line::new(self.corner_b, self.corner_d, self.rect_color.clone()).draw(canvas);
        Line::new(self.corner_d, self.corner_a, self.rect_color.clone()).draw(canvas);
    }
}

pub struct Cube {
    front_corner_a: Point,
    front_corner_b: Point,
}

impl Cube {
    pub fn new(first_point: &Point, second_point: &Point) -> Self {
        Self {
            front_corner_a: *first_point,
            front_corner_b: *second_point,
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, canvas: &mut impl Displayable) {
        let cube_color = self.color();
        let front_a = self.front_corner_a;
        let front_b = self.front_corner_b;

        let depth_x = (front_a.x - front_b.x) / 2;
        let depth_y = -((front_a.y - front_b.y) / 2);

        let mut front_face = Rectangle::new(&front_a, &front_b);
        let mut back_face = Rectangle::new(
            &(Point {
                x: front_a.x + depth_x,
                y: front_a.y + depth_y,
            }),
            &(Point {
                x: front_b.x + depth_x,
                y: front_b.y + depth_y,
            })
        );

        front_face.rect_color = cube_color.clone();
        front_face.draw(canvas);

        back_face.rect_color = cube_color.clone();
        back_face.draw(canvas);

        Line::new(front_face.corner_a, back_face.corner_a, cube_color.clone()).draw(canvas);
        Line::new(front_face.corner_b, back_face.corner_b, cube_color.clone()).draw(canvas);
        Line::new(front_face.corner_c, back_face.corner_c, cube_color.clone()).draw(canvas);
        Line::new(front_face.corner_d, back_face.corner_d, cube_color.clone()).draw(canvas);
    }
}

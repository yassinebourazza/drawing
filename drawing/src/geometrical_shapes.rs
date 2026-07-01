use raster::{Color, Image};
use rand::Rng;

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image : &mut Image);
    fn color(&self) -> Color;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    pub x : i32,
    pub y : i32,    
}

impl Drawable for Point {
    fn draw(&self, image : &mut Image){
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        Color::white()
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    pub fn random(width : i32, height : i32) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            x : rng.gen_range(0..width),
            y : rng.gen_range(0..height),
        }
    }
}

pub struct Line {
    start : Point,
    end : Point,
}

impl Drawable for Line {
    fn draw(&self, image : &mut Image) {
        draw_line(&self.start, &self.end, image, &self.color())
    }
        
    fn color(&self) -> Color {
     Color::red()
    }

}

impl Line {
    pub fn new(start : &Point, end : &Point) -> Self {
        assert!(start != end , "start and end must be differents");

        Self{
            start : start.clone(),
            end : end.clone(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        loop {
            let start = Point::random(width, height);
            let end = Point::random(width, height);
            if start != end {
                return Self { start, end };
            }
        }
    }

}

pub struct Triangle {
    p1 : Point,
    p2 : Point,
    p3 : Point,
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        
        draw_line(&self.p1, &self.p2, image, &color);
        draw_line(&self.p2, &self.p3, image, &color);
        draw_line(&self.p3, &self.p1, image, &color);
    }

    fn color(&self) -> Color {
        Color::blue()
    }
}

impl Triangle {
    pub fn new(p1 : &Point, p2 : &Point , p3 : &Point) -> Self {
        Self {
            p1 : p1.clone(),
            p2 : p2.clone(),
            p3 : p3.clone(),
        }
    }
}


fn draw_line(start: &Point, end: &Point, image: &mut Image, color: &Color) {
    let dx = (end.x - start.x).abs() as f32;
    let dy = (end.y - start.y).abs() as f32;

    let sx: f32 = if start.x < end.x { 1.0 } else { -1.0 };
    let sy: f32 = if start.y < end.y { 1.0 } else { -1.0 };

    let mut x = start.x as f32;
    let mut y = start.y as f32;

    if dy <= dx {
        let steps = dy/dx;

        for _ in 0..=dx as i32 {
            image.display(x as i32, y.round() as i32, color.clone());

            x += sx;
            y += sy * steps;
        }

    } else {
        let steps = dx/dy;

        for _ in 0..=dy as i32{
            image.display(x.round() as i32, y as i32, color.clone());
            y += sy;
            x += sx * steps;
        }
    }
}
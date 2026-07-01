use raster::{Color, Image};
use rand::prelude::*;

pub trait Drawable {
    fn draw(&self, img: &mut  Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// point
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point { 
    x: i32, 
    y: i32, 
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x: x, 
            y: y,
        }
    }
    pub fn random(img_w: i32, img_h: i32 ) -> Self {
        let mut rng = rand::thread_rng();
        Point::new(rng.gen_range(0..img_w), rng.gen_range(0..img_h))
    }
}

impl Drawable for Point {
    fn draw(&self, img: &mut  Image) {
        img.display(self.x, self.y, self.color());
    }
    fn color(&self) -> Color {
        Color::hex("#FFFFFF").unwrap()
    }
}

// circle
pub struct Circle { 
    center: Point, 
    radius: i32 
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self {
            center: center.clone(),
            radius: radius,
        }
    }
    pub fn random(img_w: i32, img_h: i32 ) -> Self {
        let mut rng = rand::thread_rng();
        
        let c = Point::random(img_w, img_h);
        let r = rng.gen_range(0..img_w);

        Self::new(&c, r)
    }
}


impl Drawable for Circle {
    fn draw(&self, img: &mut  Image) {

        let cx = self.center.x;
        let cy = self.center.y;
        let r = self.radius;

        let mut x = 0;
        let mut y = r;

        let mut er = (x * x) + (r * r) - (r * r); // x² + y² - r²

        while x <= y {
            img.display(cx + x, cy + y, self.color());
            img.display(cx - x, cy + y, self.color());
            img.display(cx + x, cy - y, self.color());
            img.display(cx - x, cy - y, self.color());
            img.display(cx + y, cy + x, self.color());
            img.display(cx - y, cy + x, self.color());
            img.display(cx + y, cy - x, self.color());
            img.display(cx - y, cy - x, self.color());

            
            x += 1;
            er += 2*x - 1;

            if er > 0 {
                y -= 1;
                er -= 2*y + 1;
            }
        } 
    }
    fn color(&self) -> Color {
        Color::hex("#15ff44").unwrap()
    }
}

//Line
pub struct Line {
    start : Point,
    end : Point,
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

impl Drawable for Line {
    fn draw(&self, image : &mut Image) {
        draw_line(&self.start, &self.end, image, &self.color())
    }
        
    fn color(&self) -> Color {
     Color::white()
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

//triangle 
pub struct Triangle {
    p1 : Point,
    p2 : Point,
    p3 : Point,
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

Rectangle
pub struct Rectangle {
     first_point: Point, 
     second_point: Point, 
} 


impl Drawable for Rectangle { 
    pub fn new(fpoint : Point , spoint : Point) -> self { 
        let left = fpoint.0.min(spoint.0); 
        let right = fpoint.0.max(spoint.0); 
        let top = fpoint.1.min(spoint.1); 
        let bottom = fpoint.1.max(spoint.1); 
        
        Self { 
            first_point: Point(left, top), 
            second_point: Point(right, bottom), 
        } 
    } 
    pub fn draw() {
        //
    } 
}
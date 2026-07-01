#![allow(dead_code)]
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
        Color::hex("#F5F5F5").unwrap()
    }
}

// circle
#[derive(PartialEq)]
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
            if (x * x) + (y * y) - (r * r) > 0 { // x² + y² - r²
                y -= 1;
            }
        } 
    }
    fn color(&self) -> Color {
        Color::hex("#E8896A").unwrap()
    }
}

//Line
pub struct Line {
    start : Point,
    end : Point,
}

impl Line {
    pub fn new(start : &Point, end : &Point) -> Self {
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
     Color::hex("#5B9BD5").unwrap()
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
        Color::hex("#9B59B6").unwrap()
    }
}



//Rectangle
pub struct Rectangle {
    first_point: Point, 
    second_point: Point, 
} 

impl Rectangle {

     pub fn new(fpoint : &Point , spoint : &Point) -> Self { 
        let x1 = fpoint.x.min(spoint.x); 
        let x2 = fpoint.x.max(spoint.x); 
        let y1 = fpoint.y.min(spoint.y); 
        let y2 = fpoint.y.max(spoint.y); 
        
        Self { 
            first_point: Point{x : x1, y : y1}, 
            second_point: Point{x : x2, y : y2}, 
        } 
    } 

}

impl Drawable for Rectangle { 
   
    fn draw(&self , image : &mut Image) {
        let x1 = self.first_point.x;
        let x2 = self.second_point.x;
        let y1 = self.first_point.y;
        let y2 = self.second_point.y;


        for x in x1..=x2 {
            image.display(x, y1, self.color());
            image.display(x, y2, self.color());
        }


        for y in y1..=y2 {
            image.display(x1, y, self.color());
            image.display(x2, y, self.color());
        }

    } 

    fn color(&self) -> Color {
        Color::hex("#7DAF7D").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_random_within_bounds() {
        for _ in 0..1000 {
            let p = Point::random(500, 500);
            assert!(p.x >= 0 && p.x < 500, "x out of bounds: {}", p.x);
            assert!(p.y >= 0 && p.y < 500, "y out of bounds: {}", p.y);
        }
    }

    #[test]
    fn line_random_within_bounds() {
        for _ in 0..1000 {
            let l = Line::random(500, 500);
            assert!(l.start.x >= 0 && l.start.x < 500, "start.x out of bounds: {}", l.start.x);
            assert!(l.start.y >= 0 && l.start.y < 500, "start.y out of bounds: {}", l.start.y);
            assert!(l.end.x >= 0 && l.end.x < 500, "end.x out of bounds: {}", l.end.x);
            assert!(l.end.y >= 0 && l.end.y < 500, "end.y out of bounds: {}", l.end.y);
        }
    }

    #[test]
    fn test_circle_properties() {
    // random
    let img_w = 800;
    let img_h = 600;

    for _ in 0..1000 {
        let circle = Circle::random(img_w, img_h);

        assert!(circle.center.x >= 0 && circle.center.x < img_w, "out of bound");
        assert!(circle.center.y >= 0 && circle.center.y < img_h, "out of bound");
    }

    // rayon = 0
    let center = Point::new(15, 15);
    let cz = Circle::new(&center, 0);

    assert!(cz.radius == 0);
    assert!(cz.center == center);

    let mut img = Image::blank(100, 100);
    cz.draw(&mut img);
}

    #[test]
    fn test_rectangle_reversed_points() {
        let p1 = Point::new(150, 300);
        let p2 = Point::new(50, 60);
        let r = Rectangle::new(&p1, &p2);

        let x1 = r.first_point.x.min(r.second_point.x);
        let y1 = r.first_point.y.min(r.second_point.y);
        let x2 = r.first_point.x.max(r.second_point.x);
        let y2 = r.first_point.y.max(r.second_point.y);

        assert!(x1 == 50, "x1 should be 50, got: {}", x1);
        assert!(y1 == 60, "y1 should be 60, got: {}", y1);
        assert!(x2 == 150, "x2 should be 150, got: {}", x2);
        assert!(y2 == 300, "y2 should be 300, got: {}", y2);
    }

    #[test]
    fn line_same_points() {
        let p = Point::new(10, 10);
        let _l = Line::new(&p, &p);
    }

}
use raster::{Color, Image};
use rand::Rng;

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image : &mut Image);
    fn color(&self) -> Color;
}

#[derive(Debug, Clone)]
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
        Self{
            start : start.clone(),
            end : end.clone(),
        }
    }

    pub fn random(width : i32, height : i32) -> Self {
        Self {
            start : Point::random(width, height),
            end : Point::random(width, height),            
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


fn draw_line(start : &Point, end :&Point, image: &mut Image, color: &Color) {
    let dx = (end.x - start.x).abs();
    let dy = (end.y - start.y).abs();

    let sx = if start.x < end.x { 1 } else { -1 };
    let sy = if start.y < end.y { 1 } else { -1 };

    let mut x = start.x;
    let mut y = start.y;

    let mut err = dx - dy;

    loop {
        image.display(x, y, color.clone());

        if x == end.x && y == end.y {
            break;
        }

        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x += sx;
        }

        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

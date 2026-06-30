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
#[derive(Clone)]
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
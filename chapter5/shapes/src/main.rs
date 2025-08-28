fn main() {
    let mut r = Rectangle::new(2.0, 3.0);
    println!("Rectangle: {r:?}");
    println!(
        "It has an area of {}, and a perimeter of {}",
        r.area(),
        r.perimeter()
    );
    r.scale(2.0);

    println!("Rectangle: {r:#?}");
    println!(
        "It has an area of {}, and a perimeter of {}",
        r.area(),
        r.perimeter()
    );
    let r2 = Rectangle::new(1.0, 8.0);
    println!("Rectangle 1 can hold Rectangle 2? {}", r.can_hold(&r2));
}

#[derive(Debug)]
struct Rectangle {
    base: f32,
    height: f32,
}

impl Rectangle {
    fn new(base: f32, height: f32) -> Self {
        Self { base, height }
    }

    fn area(&self) -> f32 {
        self.base * self.height
    }

    fn perimeter(&self) -> f32 {
        2.0 * self.base + 2.0 * self.height
    }

    fn scale(&mut self, scaling_factor: f32) {
        self.base *= scaling_factor;
        self.height *= scaling_factor;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.base >= other.base && self.height >= other.height
    }
}

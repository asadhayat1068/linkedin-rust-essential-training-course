struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, scaler: f64) {
        self.width *= scaler;
        self.height *= scaler;
    }

    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {
            width: width,
            height: height,
        }
    }
}

fn main() {
    let mut rec1 = Rectangle::new(10.0, 14.0);
    let area = rec1.get_area();
    println!("Area1: {}", area);
    rec1.scale(2.0);
    let area = rec1.get_area();
    println!("Area2: {}", area);
}

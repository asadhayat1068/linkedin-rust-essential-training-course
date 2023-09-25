enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64),
}

impl Location {
    fn display(&self) {
        match *self {
            Self::Unknown => println!("Location is Unknown"),
            Self::Anonymous => println!("Location is anonymous"),
            Self::Known(lat, lon) => println!("Location is ({lat},{lon})"),
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();

    let address = Location::Anonymous;
    address.display();

    let address = Location::Known(28.608295, -80.604177);
    address.display();
}

use std::fmt;
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

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Unknown => write!(f, "Location is Unknown"),
            Self::Anonymous => write!(f, "Location is anonymous"),
            Self::Known(lat, lon) => write!(f, "Location is ({lat},{lon})"),
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    println!("Display: {}", address);

    let address = Location::Anonymous;
    address.display();
    println!("Display: {}", address);

    let address = Location::Known(28.608295, -80.604177);
    address.display();
    println!("Display: {}", address);
}

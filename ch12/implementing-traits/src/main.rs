use std::fmt::{self};
struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "This is {} traveling at {} miles/sec",
            self.name, self.velocity,
        )
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };

    println!("{}", hubble);
}

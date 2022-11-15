struct Boeing {
    required_crew: u8,
    range: u16,
}

//This is an associated function aka method
impl Boeing {
    fn meet_the_crew(&self) { //&self mandatory
        println!("Hello! We are the crew");
    }
}

struct Airbus {
    required_crew: u8,
    range: u16,
}

//Equivalent of interface
trait Flight {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool;
}

impl Flight for Boeing {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 150 > distance) {
            true
        } else {
            false
        }
    }
}

impl Flight for Airbus {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 280 > distance) {
            true
        } else {
            false
        }
    }
}

fn main() {
    let boeing = Boeing {
        required_crew: 4,
        range: 7370,
    };
    boeing.meet_the_crew();

    let airbus = Airbus {
        required_crew: 7,
        range: 5280,
    };

    let boeing_is_legal = boeing.is_legal(boeing.required_crew, 18, boeing.range, 2356);
    let airbus_is_legal = airbus.is_legal(airbus.required_crew, 3, airbus.range, 4444);

    println!("Boeing legal: {} ", boeing_is_legal);
    println!("Airbus legal: {} ", airbus_is_legal);
}

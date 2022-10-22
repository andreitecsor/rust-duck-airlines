fn main() {
    //region OPTION
    println!("================OPTION=================");
    let phrase = String::from("Duck Airlines");
    let letter = phrase.chars().nth(4);

    let value = match letter {
        Some(character) => character.to_string(),
        None => String::from("No value")
    };

    println!("{}", value);

    //endregion OPTION

    //region MATCH
    println!("================MATCH=================");

    let animal = "Duck";
    match animal {
        "Duck" => println!("Quack"),
        "Dog" => println!("Bark"),
        _ => println!("All quiet out here") // <=> default
    }
    println!("{}", value);

    let ndb_freq: u16 = 500;
    match ndb_freq {
        200..=500 => { // is in interval [200,500]
            println!("NDB frequency is valid");
        }
        _ => {
            println!("NDB frequency is NOT valid");
        }
    }
    match ndb_freq {
        ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
            println!("NDB frequency is valid");
        }
        _ => {
            println!("NDB frequency is NOT valid");
        }
    }

    //endregion MATCH

    //region IF LET
    println!("================IF LET=================");

    let animal2 = "Cat";

    if let animal2 = "Cat" {
        println!("Meow");
    }

    //endregion IF LET
}

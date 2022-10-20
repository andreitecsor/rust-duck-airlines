#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");
    // DATA TYPES
    // Arrays & Tuples
    let _array_initialization: [u64; 2] = [0, 1_000]; // 1000 elements of unsigned int of 64bit
    let coordinates: [f64; 2] = [41.40923848, -81.8628462];
    println!("Coordinates: {} - {}", coordinates[0], coordinates[1]);
    let location = ("KCLE", 41.40923848, -81.8628462);
    let (name, latitude, longitude) = location; // Destructuring tuple
    println!("Location name: {}, latitude: {}, longitude: {}",
             name, latitude, longitude);

    // Strings and string slices
    /*
        String                    vs String slices (&str)
        --------------------------------------------------
        Vector of u3 data         || Vector of u3 data
        Mutable                   || Immutable
        Stored on the heap        || Can be stored on the heap, stack or embedded in the compiled code
     */
    let person_name_slice = "Nicu Marian";
    let person_name_string = person_name_slice.to_string();
    let person_name_string2 = String::from("Nicu Marian");

    let person_name_slice2 = &person_name_string; // & is the reference operator and can be read as “address of”.
    // in this person_name_slice2 si referencing the person_name_string from HEAP
    let person_name_slice3 = person_name_string.as_str();

    //String concatenation
    let duck = "Duck";
    let airlines = "Airlines";

    let airline_name = format!("{} {}", duck, airlines);
    println!("{}", airline_name);

    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' ');
    slogan = slogan + "every time";
    println!("{}", slogan);
}

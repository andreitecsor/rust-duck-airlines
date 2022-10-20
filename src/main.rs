#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");
    //region DATA TYPES
    println!("================DATA TYPES=================");
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

    //endregion DATA TYPES

    //region VARIABLES
    println!("================VARIABLES=================");
    //Variables
    let my_variable_name: u32 = 0;
    let my_inferred_variable = 0;
    let my_inferred_variable2 = 1_i8;
    let my_inferred_variable_f = 1.0;
    let my_inferred_variable_f2: f32 = 1.0;
    let _warning_variable = 0;

    //Casting
    let float_thirty_two: f32 = 17.2;
    let unsigned_eight: u8 = 5;
    let cast_unsigned_eight = unsigned_eight as f32;
    let result = float_thirty_two / cast_unsigned_eight;
    println!("{}", result);

    let number: u8 = 65;
    let letter: char = number as char; // translates the numerical value of the unicode character
    println!("{}", letter);

    //Mutability
    // - by default variables are immutable; to change that add the keyword "mut"
    let mut x = 32;
    println!("x= {}", x);
    x += 1;
    println!("x= {}", x);

    let y = 32;
    println!("y= {}", y);
    // y += 1; // -> Compiler error

    //Scope and shadowing
    let scope_test = "outer scope";
    println!("{}", scope_test);
    {
        let scope_test = "inner scope"; // -> shadowing scope_test
        println!("{}", scope_test); // if let scope_test = "inner scope"; will be deleted or commented this will print "outer scope"
    }
    println!("{}", scope_test);

    //endregion VARIABLES
}

fn return_greater(first: u8, second: u8) -> u8 { // -> what will return
    if first > second {
        first // skip return keyword and ;
    } else {
        second
    }
}

fn print_original(original: &String) {
    println!("fn print_original: {}", original);
}

fn change_original(original: &mut String) { // &mut means that we want to modify original
    let next = original;
    *next = String::from("next value"); //here next gets out of scope but the value at the address is modified
}

fn main() {
    println!("================FUNCTIONS=================");
    let greater = return_greater(10, 5);
    println!("{} is greater", greater);

    let mut original = String::from("original value");
    println!("Outer original: {}", original);
    {
        print_original(&original);
        change_original(&mut original);
        println!("Inner scope original: {}", original);
    }

    println!("================CLOSURES=================");
    // Closure = a function that doesnt have a name
    let write_message = || {
        println!("Hey. This is the closure.");
    };

    write_message();

    let name = "Duck Airlines";
    let print_slogan = |slogan: String| {
        println!("{} {}", name, slogan);
    };

    print_slogan(String::from("We hit the ground every time"));

    let get_slogan = |slogan: String| -> String {
        String::from(format!("{} {}", name, slogan)) // no ; means it's a return statement
    };

    let slogan = get_slogan(String::from("We hit the ground every time"));
    println!("{}", slogan);
}



#![allow(unused_variables)]

fn main() {
    println!("================OWNERSHIP=================");
    let original = String::from("original value");
    println!("original: \t\"{}\"", original);

    let next = original; // the value of let original is moved to let next that's why
    // println!("{}", original); // this displays an error at compile time

    println!("================BORROWING=================");
    let mut original = String::from("original value");
    println!("Outer original: {}", original);
    {
        let next = &mut original; // getting a reference to original value and we want
        // to able to change that value
        *next = String::from("next value"); // Dereferencing -> go to memory address of let next
        // and write "next value" there
        println!("Inner scope next: {}", next);
        println!("Inner scope original: {}", original);
    }

    println!("Outer original: {}", original);

    println!("================LIFETIMES=================");

    //DANGLING REFERENCE
    // let outer_scope;
    // {
    //     let inner_scope = 5;
    //     outer_scope = &inner_scope;
    // }
    // println!("{}", outer_scope);

    let value_one = 24;
    let value_two = 67;
    let value = explicit_lifetime(&value_one, &value_two);
    println!("{}", value);
}

fn explicit_lifetime<'a>(p1: &'a i32, p2: &'a i32) -> &'a i32 {
    if p1 > p2 {
        p1
    } else {
        p2
    }
}

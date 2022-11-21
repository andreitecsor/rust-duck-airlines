use std::ops::{Add, Div, Mul, Sub};
use std::process::Output;

#[derive(Debug)]
struct NavigationAid<T, U> {
    name: String,
    frequency: T,
    data: U,
}

fn main() {
    println!("================GENERICS================");
    let vor = NavigationAid {
        name: String::from("DQN"),
        frequency: 114.5,
        data: "some data",
    };
    let ndb_data: Option<String> = Option::None;
    let nbd = NavigationAid {
        name: String::from("HFK"),
        frequency: 239,
        data: ndb_data,
    };
    println!("VOR information is: {:?}", vor);
    println!("VOR information is: {:?}", nbd);

    println!("\n================GENERIC CONSTRAINTS================");
    let sum = add(255, 255);
    println!("{}", sum);
}

fn add<T>(operand1: T, operand2: T) -> T
    where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T> { // these are traits
    operand1 + operand2
}

// fn add<T: Add<Output = T> + Sub<Output = T> + Mul<Output= T> + Div<Output = T>>(operand1: T, operand2: T) -> T {
//     operand1 + operand2
// }
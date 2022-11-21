use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    println!("================VECTORS================");
    let mut vector: Vec<&str> = Vec::new();
    let vec_macro = vec![1, 2, 3, 4];

    vector.push("ABC1");
    vector.push("ABC2");
    vector.push("ABC3");
    vector.push("ABC4");
    vector.push("ABC5");

    for element in vector.iter() {
        println!("{}", element);
    }

    let second = vector[1]; // if index > len flights => panic
    println!("\n{}\n", second);
    let third = vector.get(6); // slower than [index] (when len > millions)
    // safer because returns an Option<>
    match third {
        Some(element) => {
            println!("{}\n", element);
        }
        _ => {
            println!("No element\n");
        }
    }

    if let Some(flight_value) = vector.get(3) {
        println!("{}\n", flight_value);
    }

    println!("\nINSERT NEW ELEMENT:");
    vector.insert(1, "NEW ELEMENT");
    for flight in vector.iter() {
        println!("{}", flight);
    }

    println!("\nREMOVE ELEMENT:");
    vector.remove(1);
    for flight in vector.iter() {
        println!("{}", flight);
    }

    println!("\nITER AND CHANGE:");
    for flight in vector.iter_mut() {
        *flight = "changed";
        println!("{}", flight);
    }

    println!("\n================VECTOR DOUBLE ENDED QUEUE (VecDeque)================");
    let mut vec_deque: VecDeque<&str> = VecDeque::new();

    vec_deque.push_front("Front 1");
    vec_deque.push_back("Back 1");
    vec_deque.push_front("Front 2");
    vec_deque.push_front("Front 3");
    vec_deque.push_back("Back 2");
    vec_deque.push_back("Back 3");

    for element in vec_deque.iter() {
        println!("{}", element);
    }

    println!("OPERATORS VALID FOR ALL VECTORS:");
    let exists = vec_deque.contains(&"Front 1"); //exact match
    println!("Element Front 1 exists: {} ", exists);
    println!("There are {} elements", vec_deque.len());
    vec_deque.clear();
    println!("There are {} elements", vec_deque.len());

    println!("================MAP================");
    //Rust has two maps:
    // 1. Hash Map
    // 2. Btree Map
    //  - used when you want to sort the keys
    let mut customers = HashMap::new();
    customers.insert("1", ("Andrei", "23"));
    customers.insert("2", ("Teo", "17"));
    customers.insert("3", ("NicuMeowe", "66"));

    let id = "1";
    let option = customers.get(id);
    let (name, age) = option.unwrap();
    println!("{},{}", name, age);

    if !customers.contains_key(&"4") {
        customers.insert("4", ("Ada", "23"));
    } else {
        println!("Customer id is already used");
    }
    println!("================SET================");
    let mut hash_set = HashSet::new();
    hash_set.insert(("Andrei", "23"));
    hash_set.insert(("Teo", "17"));
    hash_set.insert(("NicuMeowe", "66"));

    //Has no order of elements -> run multiple times to see
    for element in hash_set.iter() {
        println!("{:?}", element);
    }
}
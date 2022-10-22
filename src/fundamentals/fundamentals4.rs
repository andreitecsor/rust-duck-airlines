fn main() {
    println!("================LOOPS=================");
    println!("-> loop:");
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            continue;
        }
        println!("{}", counter);
        if counter == 10 {
            break;
        }
    }

    println!("-> while:");
    counter = 1;
    while counter <= 10 {
        println!("{}", counter);
        counter += 1;
    }

    println!("-> for:");
    for index in 1..11 {
        println!("{}", index);
    }
    println!("--> iterate:");

    let duck_aircraft = ["Boeing 737", "Boeing 767", "Boeing 787"];

    for aircraft in duck_aircraft.iter() {
        println!("{}", aircraft);
    }
}




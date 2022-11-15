use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main() {
    println!("================ERROR HANDLING================");
    let filename = "customer.json";
    match File::open(filename) {
        Ok(file) => {
            println!("{:#?}", file);
        }
        Err(error) => {
            // println!("{:#?}", error);
            match error.kind() {
                ErrorKind::NotFound => {
                    match File::create(filename) {
                        Ok(file) => {
                            println!("File created");
                        }
                        Err(error) => {
                            println!("{:#?}", error)
                        }
                    }
                }
                _ => { //equivalent of default in switch
                    println!("{:#?}", error)
                }
            }
        }
    }

    println!("================ERROR PROPAGATION================");
    let filename = "data.txt";
    let file_data = read_file(filename);
    match file_data {
        Ok(data) => {
            println!("{}", data);
        }
        Err(_) => {}
    }
}

fn read_file(filename: &str) -> Result<String, Error> {
    let mut file_handle = File::open(filename)?; //if the open functions produces an error
    //the ? at the end will return the error and exit the function
    let mut file_data = String::new();
    file_handle.read_to_string(&mut file_data)?;
    Ok(file_data)
}


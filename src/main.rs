use std::fs::File;
use std::io::ErrorKind;

fn main() {
    panic_statement();
    panic_in_library();
    println!();

    recover_with_result();
    unwrap_and_expect();
}

fn panic_statement() {
    println!(
        "Panic statement:
        \"panic!(\"We're crashing and burning!\")."
    );
}

fn panic_in_library() {
    println!(
        "Would cause panic in library: 
        let v = vec![1, 2, 3]; 
        v[99];"
    );
}

fn recover_with_result() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let _g = File::open("goodbye.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("goodbye.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn unwrap_and_expect() {
    let file_name = String::from("good-morning.txt");
    // let f = File::open(&file_name).unwrap();
    let f = File::open(&file_name).expect("Failed to open good-morning.txt.");
}

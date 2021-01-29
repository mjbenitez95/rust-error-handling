use std::fs::File;
use std::io::ErrorKind;

fn main() {
    panic_statement();
    panic_in_library();
    println!();

    recover_with_result();
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

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", error)
            }
        },
    };
}

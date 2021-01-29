use std::fs;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    panic_statement();
    panic_in_library();
    println!();

    recover_with_result();
    unwrap_and_expect();
    propagate_error();
    println!();
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

fn propagate_error() {
    read_user_name_from_file();
    read_user_name_but_concise();
    read_user_name_but_one_line();
}

fn read_user_name_from_file() -> Result<String, io::Error> {
    let file_name = String::from("username.txt");
    let f = File::open(&file_name);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_user_name_but_concise() -> Result<String, io::Error> {
    /*
        the '?' operator is the same as the Err(e) => return Err(e)
        arms of the match above, except that it'll convert any errors
        to be the type that is in the return type in the fn signature
    */
    let file_name = String::from("username.txt");
    let mut f = File::open(&file_name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s);
}

fn read_user_name_but_more_concise() -> Result<String, io::Error> {
    let file_name = String::from("username.txt");
    let mut s = String::new();

    File::open(&file_name)?.read_to_string(&mut s)?;

    Ok(s);
}

fn read_user_name_but_one_line() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

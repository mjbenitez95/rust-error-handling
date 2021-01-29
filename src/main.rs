fn main() {
    panic_statement();
    panic_in_library();
    
}

fn panic_statement() {
    panic!("We're crashing and burning!");
}

fn panic_in_library() {
    let v = vec![1, 2, 3];
    v[99];
}
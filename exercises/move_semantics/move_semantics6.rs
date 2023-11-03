// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data);  // change to borrow instead of moving
    println!("Last character is: {}", last_char);

    string_uppercase(data);  // transfer ownership
}

// Does not take ownership, uses a reference
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Takes ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{}", data);
}


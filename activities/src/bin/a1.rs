// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn display_first_name(f_name: &str) {
    println!("My first name is: {}", f_name);
}

fn display_second_name(s_name: &str) {
    println!("My second name is: {}", s_name);
}

fn main() {
    let first_name = "Simon";
    let second_name = "Sun";
    display_first_name(first_name);
    display_second_name(second_name);
}

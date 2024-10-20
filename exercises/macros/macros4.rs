// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.


#[rustfmt::skip]
macro_rules! my_macro {
    // The first rule for when no argument is passed
    () => {
        println!("Check out my macro!");
    };
    // The second rule for when an argument ($val:expr) is passed
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    // Call the macro without an argument
    my_macro!();
    // Call the macro with an argument 7777
    my_macro!(7777);
}
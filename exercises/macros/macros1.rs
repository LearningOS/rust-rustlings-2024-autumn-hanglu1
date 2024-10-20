// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.



macro_rules! greeting {
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    greeting!("Rustacean");
}

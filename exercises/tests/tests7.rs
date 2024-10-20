// tests7.rs
//
// When building packages, some dependencies can neither be imported in
// `Cargo.toml` nor be directly linked; some preprocesses varies from code
// generation to set-up package-specific configurations.
//
// Cargo does not aim to replace other build tools, but it does integrate
// with them with custom build scripts called `build.rs`. This file is
// usually placed in the root of the project, while in this case the same
// directory of this exercise.
//
// It can be used to:
//
// - Building a bundled C library.
// - Finding a C library on the host system.
// - Generating a Rust module from a specification.
// - Performing any platform-specific configuration needed for the crate.
//
// When setting up configurations, we can `println!` in the build script
// to tell Cargo to follow some instructions. The generic format is:
//
//     println!("cargo:{}", your_command_in_string);
//
// Please see the official Cargo book about build scripts for more
// information:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// In this exercise, we look for an environment variable and expect it to
// fall in a range. You can look into the testcase to find out the details.
//
// You should NOT modify this file. Modify `build.rs` in the same directory
// to pass this exercise.
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.


use std::env;

fn main() {
    // Check for the environment variable `TEST_ENV`
    match env::var("TEST_ENV") {
        Ok(value) => {
            // Attempt to parse the value as an integer
            if let Ok(number) = value.parse::<i32>() {
                // Check if the number is within the desired range
                if (1..=100).contains(&number) {
                    // If it is, set `TEST_FOO` to "1"
                    println!("cargo:rustc-env=TEST_FOO=1");
                } else {
                    // If it's not, set `TEST_FOO` to "0"
                    println!("cargo:rustc-env=TEST_FOO=0");
                }
            } else {
                // If parsing fails, set `TEST_FOO` to "0"
                println!("cargo:rustc-env=TEST_FOO=0");
            }
        }
        Err(_) => {
            // If the environment variable is not set, set `TEST_FOO` to "0"
            println!("cargo:rustc-env=TEST_FOO=0");
        }
    }
}




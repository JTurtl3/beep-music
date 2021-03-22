mod note;
pub use note::*;

mod parser;
pub use parser::*;

use std::process::Command;

pub fn beep(frequency: u32, length: u32) {
    Command::new("beep")
        .args(&[
            "-f", frequency.to_string().as_str(),
            "-l", length.to_string().as_str()]
        )
        .status()
        .expect("Oh no :(");
}

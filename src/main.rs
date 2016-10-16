extern crate rush;
use rush::{get_line, run_command};

pub fn main() {
    while let Some(line) = get_line("$ ") {
        run_command(line);
    }
}


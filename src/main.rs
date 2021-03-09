use std::process::{ Command };

fn main() {
    Command::new("hedge://open")
                .spawn()
                .expect("Failed to open Hedge.");
}

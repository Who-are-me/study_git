use std::process::Command;
use std::io::{self, Write};

mod my_mod;

fn main() {
    println!("Hello from test Rust project for study git!");
    my_mod::fn_from_my_mod();
    my_mod::my_mod::fn_from_my_mod_in_my_mod();

    let output_from_python = Command::new("python3")
        .arg("src/sp.py")
        .output()
        .expect("Not run Python script");

    println!("\nStatus code from python script: {}", output_from_python.status);

    io::stdout().write_all(&output_from_python.stdout).unwrap();
    io::stderr().write_all(&output_from_python.stderr).unwrap();
}

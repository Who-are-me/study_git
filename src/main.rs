use std::process::Command;


mod my_mod;


fn main() {
    println!("Hello from test Rust project for study git!");
    my_mod::fn_from_my_mod();
    my_mod::my_mod::fn_from_my_mod_in_my_mod();
    Command::new("python").arg("sp.py");
}

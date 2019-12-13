// This library contains the full cybus game engine.

#![crate_name = "cybus"]

pub fn hello_world() -> String {
    let greeting = String::from("Hello World!");
    println!("{}", greeting);
    greeting
}

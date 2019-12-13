// This library contains the full Cybus game engine.

#![crate_name = "cybus"]

mod cybus;

use log::debug;

pub fn init(verbosity: u8) {
    // Verbosity of the logger (0-4):
    // 0 => Error
    // 1 => Warn
    // 2 => Info
    // 3 => Debug
    // 4 => Trace

    cybus::logger::setup_logger(verbosity).expect("ERROR: Failed to initialize logging.");
    debug!("Logger Initialized");
}

pub fn hello_world() -> String {
    let greeting = String::from("Hello World!");
    println!("{}", greeting);
    greeting
}

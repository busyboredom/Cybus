// This is a dummy application for testing the library

extern crate cybus;

fn main() {
    cybus::hello_world();

    // Initialize cybus core components and set logging verbosity to 4.
    cybus::init(4);
}

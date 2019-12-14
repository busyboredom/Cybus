// Test that an application can run a cybus function.

#[test]
fn hello_world() {
    assert_eq!(String::from("Hello World!"), cybus::hello_world());
}

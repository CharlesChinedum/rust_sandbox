// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // Primitive str
    let hello = "Hello Primitive str";

    // String
    let mut hello_string = String::from("Hello String ");

    // Get length - works for both primitive str and String
    println!("Length: {}", hello.len());

    // Adding to a string
    // push - push a character to the end of the string
    hello_string.push('W');

    // push_str - push a string slice to the end of the string
    hello_string.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello_string.capacity());

    // Check if empty
    println!("Is Empty: {}", hello_string.is_empty());

    // Contains
    println!("contains 'word' {}", hello_string.contains("World"));

    // Replace
    println!("Replace: {}", hello_string.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello_string.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    println!("{}", hello_string);
}

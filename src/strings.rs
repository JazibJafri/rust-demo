// Primitive str = Immutable fixed-length string
// String = Growable, heap-allocated structure, use when you need to modify string

pub fn run() {
    //mutable string
    let mut hello = String::from("Hello ");

    // get length
    println!("Length {}", hello.len());

    // push char
    hello.push('W');

    // push string
    hello.push_str("orld");

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // check if empty
    println!("Is Empty: {}", hello.is_empty());

    // contains
    println!("Contains 'World' {}", hello.contains("World"));
    
    // replace
    println!("Replace 'World' {}", hello.replace("World", "There"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');


    // assertion testing
    assert_eq!(2, s.len());
    
    assert_eq!(10, s.capacity());

    println!("{}", s);
}

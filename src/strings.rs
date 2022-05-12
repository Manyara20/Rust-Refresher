//primitive str = immutable fixed-length string somewhere in memory
//String = Growable, heap-allocated data-structure - Use when you need to modify or own string data
//Push method & push string for the char type

pub fn run(){
    
    let mut hello =  String::from("Hello ");

    //Get length
    println!("Length: {}", hello.len());

    //Push string
    hello.push_str("World");

    //Push char
    hello.push('\u{1F600}');
    
    //Capacity in bytes
    println!("Capacity:{}", hello.capacity());

    //Check if string is empty

    println!("Is Empty:{}", hello.is_empty());

    // Cointains substring
    println!("Cointains 'World' {}", hello.contains("World"));

    //Replace
    println!("Replace:{}", hello.replace("World", "Silva"));

    //Loop through string by whitespace
    for word in hello.split_whitespace(){

        println!("{}", word);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    //Assertion testing
    assert_eq!(2, s. len());
    assert_eq!(10, s. capacity());

    println!("{}", s);


    
    

}
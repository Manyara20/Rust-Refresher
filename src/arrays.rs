//Arrays- Fixed lists where elements are the same data types
use std::mem;

pub fn run(){

    let mut numbers: [i32; 5] = [1,2,3,4,5];

    //Re-assign value
    numbers[3] = 300;
    
    println!("{:?}", &numbers);

    //Get a single value

    println!("Single value: {}", &numbers[0]);

    //Get array length
    println!("Array length is: {:?}", numbers.len());

    //Arrays are stack allocated
    println!("Arrays occupies: {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);



}
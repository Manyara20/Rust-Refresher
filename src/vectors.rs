//Vectors - Resizable arra

    
use std::mem;

pub fn run(){

    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    //Re-assign value
    numbers[3] = 100;

    //Add on to vector
    numbers.push(50);
    numbers.push(90);

    //Pop off last value
    numbers.pop();
    
    println!("{:?}", &numbers);

    //Get a single value

    println!("Single value: {}", &numbers[0]);

    //Get vector length
    println!("Vector length is: {:?}", numbers.len());

    //Vector are stack allocated
    println!("Vector occupies: {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);

    //Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);

    }

    //Loops & mutate values
    for x in numbers.iter_mut(){
        *x *=2;

    }

    println!("Numbers Vec: {:?}", numbers);


}




//Variables hold primitive date or references to data
//Variables are immutable by default
//Rust is a block-scoped language


pub fn run(){
 let name = "Ezra";
 let mut age = 37;
 println!("My name is {} I am {}", name, age);
age = 38;

 println!("My name is {} I am {}", name, age);

 //Define constant
 const ID: i32 =001;
 println!("ID: {}", ID);

 //Assign multiple vars
 let ( my_name, my_age ) = ("Audry", 22);
 println!("My name is {} I am {}", my_name, my_age);

}
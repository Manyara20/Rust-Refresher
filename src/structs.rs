//Structs - used to create custom data types 

//Traditional struct

// struct Color{
// red: u8,
// green: u8,
// blue: u8
// }

//Tuple Struct

// struct  Color(u8, u8, u8);

struct Person{
first_name: String,
second_name:String,
pub age:u8,

}

impl  Person {
    fn new(first: &str, last: &str, _age:u8) -> Person{
    Person{

    first_name: first.to_string(),
    second_name: last.to_string(),
    age: 20
    
    
    
    }
    }
    //Get full name
    fn full_name(&self) -> String{
        format!("{} {} {}", self.first_name, self.second_name, self.age)
    }
    //Set last name
    fn set_last_name(&mut self, last: &str) {
        self.first_name = last.to_string();
    }
    //Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.second_name)
    }
}
pub fn run(){
// let mut c = Color {
// red: 255,
// green: 0,
// blue: 0
// };
// c.red = 200;

// println!("Color: {} {} {} ", c.red, c.green, c.blue);  

// let mut c = Color(255,0,0);

// c.0=200;
// println!("Color: {:?} {:?} {:?}", c.0, c.1, c.2);

let mut p = Person::new("Ezra" , "Manyara", 20);
println!("Person {} {} {}", p.first_name, p.second_name, p.age);
println!("Person {}", p.full_name());
p.set_last_name("Okenda");
println!("Person {}", p.full_name());
println!("Person Tuple {:?}", p.to_tuple());

}
//Conditionals- Used to check the condition of something and act on the result
pub fn run(){

    let age = 12;
    let check_id = true;
    let knows_person_of_age = true;

    //If/Else
    if age >=21 && check_id || knows_person_of_age {

        println!("Bartender: What would you like to drink");

    } else if age < 21 && check_id {
        println!("Below age! Go home and have a glass of water");
    }else {
        println!("Bartender: I'll like to see your ID");
        
    }

    //Shorthand If
    let is_of_age = if age >= 21 {true} else {false};
    print!("Is of age: {}", is_of_age);



}
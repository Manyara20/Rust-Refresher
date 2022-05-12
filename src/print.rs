pub fn run(){
    
    //print to console
    println!("Hello from Ezra");
    
    
    println!("Number: {}", 1);

    //Basic Formatting
    println!("{} is from {}", "Ezra" , "Mass");

    //Positional Arguements
    println!("{0} is from {1} and {0} likes to {2}", "Ezra" , "Mass", "code");

    //Named Arguements
    println!("{name} likes to play {activity}", name = "Ezra", activity="Footballs");

    //Placeholder traits
    println!("Binary: {:b} Hex:{:x} Octal:{:o}", 10,10,10);
    //Placeholder for debug trait
    println!("{:?}",(12, true,"sasa"));
    //Basic math
    println!("10+10 = {}", 10+10);
    //Basic math
    println!("10*10 = {}", 10*10);



}
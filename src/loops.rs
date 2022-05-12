//Loops - Used to iterate until a condition is met


pub fn run(){
    let mut count = 0;
    

    //Infinite Loop
    // loop{

    //     count += 1;
    //     println!("Number: {}", count);

    //     if count==10{
    //     break;

    //     }
       
    // }

    //WhileLoop (FizzBuzz)

    // while count <= 100{
    //     if count % 15 ==0{
    //         println!("FizzBuzz");
    //     }else if count % 3 ==0 {
    //         println!("Fizz");
            
    //     }else if count % 5 ==0 {
    //         println!("Buzz");
    //     }else if count == 49 {
    //         println!("Ezra")
            
    //     } 
    //     else {
    //         println!("{}", count);
    //     }
    //      //Increament
    // count += 1;
    // }
   
    //For Range loop

    for x in 0..100 {

        if x % 15 ==0{
            println!("FizzBuzz");
        }else if x % 3 ==0 {
            println!("Fizz");
            
        }else if x % 5 ==0 {
            println!("Buzz");
        }else if x == 49 {
            println!("Ezra")
            
        } 
        else {
            println!("{}", x);
        }
    }


}
pub fn run(){

    //Check number if it is Odd or Even
    let a=88889;
    if a%2==0{
        println!("Even");
    }
    else{
        println!("Odd");
        //Loops 
        for i in 0..9 { // 0..7 is range expression including 0 excluding 7.
            println!("variable `i` is : {}", i);
        }
    }
}
//Enums are types which have a few definite values

enum Movement 
  {
    
    //Variant
    Up,
    Down,
    Right,
    Left,
   }
   fn move_avatar(m:Movement){
       //Perform action depending on info
       match m {
           Movement::Up=> println!("Avatar moving up"),
           Movement::Down=> println!("Avatar moving Down"),
           Movement::Right=> println!("Avatar moving Right"),
           Movement::Left=> println!("Avatar moving Left"),
           
       }
   }

pub fn run(){
let avatar1=Movement::Left;
let avatar2=Movement::Right;
let avatar3=Movement::Up;
let avatar4=Movement::Down;

move_avatar(avatar1);
move_avatar(avatar2);
move_avatar(avatar3);
move_avatar(avatar4);



}
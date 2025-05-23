use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main(){
    let crct_numb:u32=rand::thread_rng().gen_range(1..=100);
    
    loop{
        println!("please enter your guess");
        let mut guess=String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to get line");

        let guess:u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        println!("you guessed {}",guess);
        
        match guess.cmp(&crct_numb){
            Ordering::Less=>println!("too small"),
            Ordering::Greater=>println!("too big"),
            Ordering::Equal=>{
                println!("yahooo! thats the correct one");
                break;}

        }

    }



 
}
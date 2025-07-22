extern crate rand;
use std::io;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;
fn main() 
{
    println!("Guess the number between 1-2048, You have 10  chances to guess it:");
    println!("Please enter your guess:");
    let secret_number = rand::thread_rng().gen_range(1,2048);
    let mut lifecount = 11;
    
    
    loop{
         let mut guess = String::new(); 
         lifecount -= 1;
         println!("LIFE:{}",lifecount);
            
             if lifecount == 0 {
                 println!("GAME OVER, BRO! The secret number is {}",secret_number);
                 break;
             }
         print!("Your guess:{}",guess); 
         let _= io::stdout().flush(); // correct the lines for the user's input
         io::stdin().read_line(&mut guess).expect("Failed to read line");
                  
            let guess: u32 = match guess.trim().parse(){
                Ok(num) => num,   
                Err(_)  => continue,
            };
        

            match guess.cmp(&secret_number){
                Ordering::Less      => println!("It's LOW"),
                Ordering::Greater   => println!("It's HIGH"),
                
                Ordering::Equal     => { println!("You are right! the secret number is {}",secret_number); break; }
                
                
               }

            // println!("The secret number is {}", secret_number);

        } 
}

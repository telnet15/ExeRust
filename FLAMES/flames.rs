use std::io;
// use std::io::Write;

fn main(){
    let  mut your_name = String::new();
    let  mut other_name  = String::new();
    
    println!("Enter your name");
    io::stdin().read_line(&mut your_name).expect("Error Reading Line!");


    println!("other  name");
    io::stdin().read_line(&mut other_name).expect("Error Reading Line!");

    

  //  let mut char_count1 :u32  = your_name.trim().parse().expect("Error parsing 1");
  //  let mut char_count2 :u32  = other_name.trim().parse().expect("Error parsing 1");
     
    let char_count1_usize = your_name.trim().chars().count();
    let char_count2_usize = other_name.trim().chars().count();

    let  char1_u32 = char_count1_usize as u32;
    let  char2_u32 = char_count2_usize as u32;
    
    let mut counter :u32  = char1_u32 + char2_u32;


    while counter > 6 {

    counter  = counter-6;

            }   

    if counter == 1 {

        println!("You will be Friends");

    } 
      if counter == 2 {

        println!("You will be Lovers");

    } if counter == 3 {

        println!("You will be Anger");

    } if counter == 4 {

        println!("You will be Married");

    } if counter == 5 {

        println!("You will be Enemies");

    } if counter == 6 {

        println!("You will be Singles");

    }
}

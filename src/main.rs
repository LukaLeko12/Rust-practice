
use std::{io};
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main()  {

    let mut low = String::new(); // "5"
    let mut high = String::new();

    io::stdin()
        .read_line(&mut low) 
        .expect("Failed to read line");

        io::stdin()
        .read_line(&mut high) 
        .expect("Failed to read line");


    let low = low.trim().parse::<u32>().unwrap(); //5
    let high = high.trim().parse::<u32>().unwrap();

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(low,high);

    

 loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; 

     println!("You guessed: {}", guess);

     match guess .cmp(&secret_number) {
        Ordering::Less => println!("{}", "Too small!".red()),
        Ordering::Greater => println!("{}", "Too big!".red()),
        Ordering::Equal => {
            println!("{}", "You win!".green());
            break;
        },
     } 
         
     

         

}

}



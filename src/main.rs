use colored::*;
use rand::Rng;
use std::io;
fn main() {
    println!("Dice Game!");
    println!("Player 1's name: ");
    let mut p1_name = String::new();
    let mut p2_name = String::new();

    io::stdin()
        .read_line(&mut p1_name)
        .expect("Failed to read line");

    println!("Player 2's name: ");

    io::stdin()
        .read_line(&mut p2_name)
        .expect("Failed to read line");

    loop {
        let p1_number = rand::thread_rng().gen_range(1..6);
        let p2_number = rand::thread_rng().gen_range(1..6);

        println!("{}{}{}", p1_name.blue(), " Rolled a ", p1_number);
        println!("{}{}{}", p2_name.purple(), " Rolled a ", p2_number);

        if p1_number > p2_number {
            println!("{}{}", p1_name.blue(), " Won!".green());
        } else if p1_number < p2_number {
            println!("{}{}", p2_name.purple(), " Won!".green());
        } else {
            println!("{}", "Game was tied!".yellow());
        }
        println!("Play again?");
        println!(
            "Press any button to play again \n
        Press 1 to quit"
        );
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if user_input == 1 {
            break;
        }
    }
}

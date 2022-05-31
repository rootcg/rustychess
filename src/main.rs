use std::{io::{self, Write, Error}, str::FromStr};

pub mod chess;

fn main() -> std::io::Result<()> {
    println!("Welcome to the chess notation parser writen in Rust! 🦀");
    println!("You could introduce a move using algebraic notation and it will tell you if it is valid.");
    println!();

    let mut try_again: bool = true;
    while try_again {
        print!("Chess move: ");
        io::stdout().flush()?;
    
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the move");
    
        let chess_move = input.trim();
        match chess::moves::Move::from_str(chess_move) {
            Ok(m) => println!("{} is a valid move! It is represented like: {:#?}", &chess_move, m),
            Err(e) => println!("{} is not a valid move! The reason is: {:#?}", &chess_move, e)
        }

        try_again = read_try_again()?;
        println!()
    }

    Ok(())
}

fn read_try_again() -> Result<bool, Error>  {
    println!("Do you want to try again?");
    
    let mut try_again: Option<bool> = None;
    while try_again.is_none() {
        print!("(y/n): ");
        io::stdout().flush()?;
        
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read the answer");
            
        try_again = match answer.trim() {
            "y" => Some(true),
            "n" => Some(false),
            _ => continue
        };
    }

    Ok(try_again.unwrap())
}
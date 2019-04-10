extern crate tictactoe;

use std::io;

use tictactoe::Game;

fn main() {
    let mut game = Game::new();
    
    loop {
        println!("{}'s turn", game.get_cur_player());
        println!("========");
        game.display_board();
        println!();

        let choice: usize = match get_choice() {
            Ok(num) => num,
            Err(info) => {
                println!("{}", info);
                continue;
            } 
        };

        let choice = choice - 1; // Change from 1-indexed to 0-indexed

        match game.make_move(choice) {
            Ok(_) => {},
            Err(info) => {
                println!("{}", info);
                continue;
            }
        };

        println!();

        let winner = game.check_win();

        if winner != ' ' {
            println!("{} is the winner!", winner);
            break;
        } else if game.check_filled() {
            println!("It's a draw!");
            break;
        }
    }
    game.display_board();
}

fn get_choice() -> Result<usize, &'static str> {
    println!("Pick a square by it's number: ");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
        .expect("Failed to read line.");
    
    let choice: usize = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err("Not a valid number!"),
    };

    if choice > 9 || choice < 1{
        Err("Not a valid number!")
    } else {
        Ok(choice)
    }
}

extern crate tictactoe;

use std::io;

use rand::{thread_rng, Rng};

use tictactoe::Game;

enum GameType {
    PlayerVsPlayer,
    PlayerVsCPU,
}

fn main() {
    let game_type = match get_game_type() {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match game_type {
        GameType::PlayerVsPlayer => pvp_game(),
        GameType::PlayerVsCPU => pve_game(),
    }
}

fn pve_game() {
    let mut game = Game::new();

    loop {
        let cur_player = game.get_cur_player();
        match cur_player {
            'X' => {
                println!("Your turn ({})", cur_player);
                println!("=============");
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
                    Ok(_) => {}
                    Err(info) => {
                        println!("{}", info);
                        continue;
                    }
                };
            }
            _ => {
                println!("Computer's turn ({})", cur_player);
                println!("===================");
                game.display_board();
                println!();

                let choices = game.get_free_spaces();

                let random_index =
                    (thread_rng().gen::<f64>() * choices.len() as f64).floor() as usize;
                let choice = choices[random_index];

                match game.make_move(choice) {
                    Ok(_) => {}
                    Err(info) => {
                        println!("{}", info);
                        continue;
                    }
                };
            }
        }
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

fn pvp_game() {
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
            Ok(_) => {}
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

fn get_game_type() -> Result<GameType, &'static str> {
    println!("Pick game type:");
    println!("1. player vs. player");
    println!("2. player vs. computer");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line.");

    match choice.as_ref() {
        "1\n" => Ok(GameType::PlayerVsPlayer),
        "2\n" => Ok(GameType::PlayerVsCPU),
        _ => Err("Invalid choice!"),
    }
}

fn get_choice() -> Result<usize, &'static str> {
    println!("Pick a square by it's number: ");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line.");

    let choice: usize = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err("Not a valid number!"),
    };

    if choice > 9 || choice < 1 {
        Err("Not a valid number!")
    } else {
        Ok(choice)
    }
}

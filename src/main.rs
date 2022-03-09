
use std::env;
use std::num::ParseIntError;
use mini_games::*;
use std::io;
use std::io::Read;
use mini_games::Status;

/*
let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    odd_or_even(args)
 */


fn main() {
    let mut deck = Deck::new();
    let mut user = Player::new();

    deck.shuffle();
    deck.deal(2, &mut user);
    println!("{}", user);
    //println!("DECK: \n{}", deck);

    loop {
        println!("Your move (type 'h' / 's' for 'hit' / 'stick'):");
        let mut response = String::new();
        io::stdin().read_line(&mut response);
        //println!("{:?}", response); //echo
        let trimmed_response = response.trim();
        if trimmed_response.trim().len() != 1 {
            println!("must be a single letter ( 'h' or 's' )")
        } else if trimmed_response.contains("h") {
            println!("hit!");
            deck.deal(1, &mut user);
        } else if trimmed_response.contains("s") {
            println!("stick!");
            user.status = Status::Stick;
        };
        user.update();
        match user.status {
            Status::Bust => {
                println!("Bust!");
                break;
            },
            Status::Stick => {
                break;
                //compare to dealers and do bets
            }
            _ => println!("{}", user)
        }


    }




    println!("{}", user);
    /*

    match user.status {
        Status::Bust => println!("BUST!"),
        Status::Hit => deck.deal(1, &mut user)

    }
    while user.status. Status::Bust {
        ;
        println!("cards: {}, total: {}", user.hand, user.total)
    }
*/


}


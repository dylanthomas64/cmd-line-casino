use std::env;
use std::num::ParseIntError;
use std::fmt;
use rand::prelude::*;

/*
pub fn odd_or_even(args: Vec<String>) {
    let arg: Result<i32, ParseIntError> = args[1].parse();
    match arg {
        Ok(number) => if number % 2 == 0 {
            println!("{} is EVEN!", number)
        } else {
            println!("{} is ODD!", number)
        },
        Err(e) => println!("Error: {}", e)
    }
}
*/

pub struct Player {
    pub hand: Deck,
    pub total: usize,
    pub status: Status
}


pub enum Status {
    Hit,
    Stick,
    Bust
}

pub struct Card (String, String);

pub struct Deck {
    pub cards: Vec<Card>
}


impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck { cards: Vec::with_capacity(52), };
        let suits = vec!["H", "S", "D", "C"];
        let values = vec!["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
        for suit in &suits {
            for value in &values {
                deck.cards.push(
                    Card (String::from(value.clone()), suit.clone().to_string())
                );
                }
        }
        deck
    }

    pub fn new_empty_deck() -> Deck {
        Deck {
            cards: Vec::new(),
        }
    }

    pub fn deal(&mut self, cards: usize, player: &mut Player) {
        for card in 0..cards {
            match self.cards.pop() {
                Some(card) => {
                    player.hand.cards.push(card);
                    player.update();
                },
                _ => println!("Deck is empty")
            }
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

impl Player {
    pub fn new() -> Player {
        let status = Status::Stick;
        Player {
            hand: Deck::new_empty_deck(),
            total: 0,
            status,
        }
    }

    /*
    pub fn update(&mut self) {
        self.total = 0;
        for Card(value, suit) in self.hand.cards.iter() {
            if value == &"A".to_string() {
                self.total += 11;
            } else if "JQK".contains(&value) {
                self.total += 10;
            } else {
                    let value = value.parse::<usize>().unwrap();
                    self.total += value;
                }

        }
        if self.total > 21 {
            self.status = Status::Bust
        }
    }
    */


    pub fn update(&mut self) {
        self.total = 0;
        for Card(value, suit) in self.hand.cards.iter() {
            match value.as_str() {
                "A" => self.total +=11,
                "J" | "Q" | "K" => self.total +=10,
                _ => {
                    let number: usize = value.parse().unwrap();
                    self.total += number;
                }
            }
        }
        if self.total > 21 {
            self.status = Status::Bust
        }

    }
}



impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list: String = String::new();
        for card in &self.cards {

           list.push_str(&format!("{} ", card));
        }
        write!(f, "{}", list)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hand: {} total: {} status: {}", self.hand, self.total, self.status)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m = match self {
            Status::Stick => "Stick",
            Status::Bust => "Bust",
            Status::Hit => "Hit",
        };
        write!(f, "{}", m)

    }
}

/*
impl Iterator for Deck {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item>{
        self.cards.
    }
}*/
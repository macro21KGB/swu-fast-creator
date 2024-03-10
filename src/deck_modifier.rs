use std::io::{self, Write};

use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct Deck {
    deck: Vec<Card>,
    leader: Card,
    base: Card,
    sideboard: Vec<Card>,
    metadata: Metadata,
}

#[derive(Serialize, Deserialize, Clone)]
struct Card {
    id: String,
    count: i32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Metadata {
    name: String,
    author: String,
}

fn convert_id_to_card_name(id: &String) -> String {
    // read json file
    let file = std::fs::File::open("cards.json").unwrap();
    let cards: HashMap<String, String> = serde_json::from_reader(file).unwrap();

    // get card name from id
    let card_name = cards.get(id).unwrap();

    card_name.to_string()
}
pub fn create_new_deck() {
    println!("Creating a new deck");

    let mut name_of_deck = String::new();
    print!("Enter the name of the deck: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name_of_deck).unwrap();

    let name_of_deck = name_of_deck.trim();

    let mut cards: HashMap<String, i32> = HashMap::new();

    loop {
        let mut id_of_card = String::new();
        print!("Enter the id of a card (or leave blank to finish): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut id_of_card).unwrap();

        if id_of_card.trim().is_empty() {
            break;
        }

        let id_of_card = id_of_card.trim();

        cards
            .entry(id_of_card.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    println!(
        "Creating a new deck named {} with the following cards:",
        name_of_deck
    );
    for (id, count) in cards.iter() {
        println!(
            "{} copy of {}({})",
            count,
            convert_id_to_card_name(id),
            id.bold()
        );
    }
}

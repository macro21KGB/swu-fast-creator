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

impl Deck {
    fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(err) => panic!("Error converting deck to json: {}", err),
        }
    }
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

fn save_deck_to_file(deck: &Deck) {
    let json = deck.to_json();
    let file_name = format!("{}.json", deck.metadata.name);
    std::fs::write(file_name, json).unwrap();
}

fn prepend_sor_to_id(id: &String) -> String {
    let mut pack_id = "SOR_".to_string();
    pack_id.push_str(&add_zero_to_id(id));
    pack_id
}

fn add_zero_to_id(id: &String) -> String {
    // given the id "12", return "012"
    // given the id "1", return "001"
    let mut new_id = id.clone();
    if id.len() == 1 {
        new_id = format!("00{}", id);
    } else if id.len() == 2 {
        new_id = format!("0{}", id);
    }
    new_id
}
pub fn create_new_deck() {
    println!("Creating a new deck");

    let mut name_of_deck = String::new();
    print!("Enter the name of the deck: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name_of_deck).unwrap();

    let name_of_deck = name_of_deck.trim();

    let mut cards: HashMap<String, i32> = HashMap::new();

    // leader card
    let mut leader_id = String::new();
    print!("Enter the id of the leader card: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut leader_id).unwrap();

    let leader_card = Card {
        id: prepend_sor_to_id(&leader_id.trim().to_string()),
        count: 1,
    };

    // base card
    let mut base_id = String::new();
    print!("Enter the id of the base card: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut base_id).unwrap();

    let base_card = Card {
        id: prepend_sor_to_id(&base_id.trim().to_string()),
        count: 1,
    };

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

    // convert cards to Vec<Card>
    let deck = cards
        .iter()
        .map(|(id, count)| Card {
            id: prepend_sor_to_id(id),
            count: *count,
        })
        .collect::<Vec<Card>>();

    let new_deck = Deck {
        metadata: Metadata {
            name: name_of_deck.to_string(),
            author: "Anonymous".to_string(),
        },
        deck,
        leader: leader_card,
        base: base_card,
        sideboard: vec![],
    };

    save_deck_to_file(&new_deck);
}

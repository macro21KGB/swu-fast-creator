use colored::*;
use std::io::{self, Write};

mod deck_modifier;

fn print_ascii_art() {
    println!(
        r"
 _____  _    _  _   _                            
/  ___|| |  | || | | |                           
\ `--. | |  | || | | |                           
 `--. \| |/\| || | | |                           
/\__/ /\  /\  /| |_| |                           
\____/  \/  \/  \___/                            
______   ___   _____  _____                      
|  ___| / _ \ /  ___||_   _|                     
| |_   / /_\ \\ `--.   | |                       
|  _|  |  _  | `--. \  | |                       
| |    | | | |/\__/ /  | |                       
\_|    \_| |_/\____/   \_/                       
 _____ ______  _____   ___   _____  _____ ______ 
/  __ \| ___ \|  ___| / _ \ |_   _||  _  || ___ \
| /  \/| |_/ /| |__  / /_\ \  | |  | | | || |_/ /
| |    |    / |  __| |  _  |  | |  | | | ||    / 
| \__/\| |\ \ | |___ | | | |  | |  \ \_/ /| |\ \ 
 \____/\_| \_|\____/ \_| |_/  \_/   \___/ \_| \_|
"
    );
}

fn print_main_menu() {
    println!("[1] {}iew/{}odify decks", "V".bold(), "M".bold());
    println!("[2] {}xit", "E".bold());
}

enum MenuOption {
    ViewModifyDecks,
    Exit,
    Invalid,
}

// View/Modify Decks ------------------------------
fn view_modify_decks_page() {
    println!("View/Modify Decks");
    // create a new deck
    println!("[1] {}reate a new deck", "C".bold());
    println!("[2] {}iew existing decks", "V".bold());
    println!("[3] {}xit", "E".bold());

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    match input.as_str() {
        "1" | "c" => deck_modifier::create_new_deck(),
        "2" | "v" => view_existing_decks(),
        "3" | "e" => {
            io::stdout().flush().unwrap();
        }
        _ => println!("Invalid input"),
    }
}

// Create a new deck

fn view_existing_decks() {
    println!("Viewing existing decks");
}

fn main() -> Result<(), io::Error> {
    let is_running = true;

    while is_running {
        print_ascii_art();
        print_main_menu();

        // wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        input = input.trim().to_string();
        let result = match input.as_str() {
            "1" | "v" | "m" => MenuOption::ViewModifyDecks,
            "2" | "e" => MenuOption::Exit,
            _ => MenuOption::Invalid,
        };

        match result {
            MenuOption::ViewModifyDecks => view_modify_decks_page(),
            MenuOption::Exit => {
                println!("Exiting...");
                break;
            }
            MenuOption::Invalid => {
                Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"))?
            }
        }
    }

    Ok(())
}

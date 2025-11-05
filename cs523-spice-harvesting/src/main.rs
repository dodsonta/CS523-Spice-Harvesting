mod item;
use item::Item;
mod gamestate;
use gamestate::GameState;
use std::io::{self, Write};
use std::time::SystemTime;

fn main() {
    let items = vec![
        Item::new("Tools", 0, 1, 10),
        Item::new("Fremen", 0, 2, 50),
        Item::new("Spice Harvester", 0, 10, 500),
    ];
    let mut game_state = GameState::new(items);
    let mut curr_time = SystemTime::now();
    let mut input = String::new();
    loop {
        print!("Enter Command: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let cmd = input.trim();
        match cmd {
            "" => {
                game_state.update_spice_by_flat(1);
                game_state.update_spice(&mut curr_time);
            }
            "Inventory" => {
                game_state.list_inventory();
            }
            "Shop" => {
                game_state.list_shop();
                print!("Enter item number to purchase or press Enter to cancel: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let input = input.trim();
                if input.is_empty() {
                    continue;
                }
                let item_num = input.parse::<usize>();
                match item_num {
                    Ok(i) => {
                        if i == 0 || i > game_state.num_items() {
                            println!("Invalid item number");
                            continue;
                        }
                        game_state.buy_item(i - 1);
                    },
                    Err(_) => {
                        println!("Invalid input");
                    }
                }
            },
            "exit" => break,
            _ => {
                println!("Unknown command");
            }
        }
        println!("Spice: {}", game_state.get_spice());
    }
}

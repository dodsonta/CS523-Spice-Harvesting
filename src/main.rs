mod item;
use item::Item;
mod userstate;
use userstate::UserState;
use std::fs::File;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use ggez::*;
use ggez::graphics;

//I realize ggez has it's own save system, but I already had this implemented before I decided to use ggez
// and didn't want to figure out how to change it
fn save_game(user: &mut UserState) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
    user.set_time_last_updated(now);
    let file = File::create("savegame.json").expect("Unable to open or create file");
    serde_json::to_writer(file, user).expect("Unable to write game state to file");
}

fn load_game() -> Option<UserState> {
    let file = File::open("savegame.json").ok()?;
    let user_state: UserState = serde_json::from_reader(file).ok()?;
    Some(user_state)
}

// fn main() {
//     //Check if there's a save, if not start new game
//     let mut game_state = match load_game() {
//         Some(state) => {
//             println!("Loaded saved game.");
//             println!("Inventory:");
//             state.list_inventory();
//             state
//         }
//         None => {
//             let items = vec![
//                 Item::new("Tools", 0, 0.1, 15),
//                 Item::new("Fremen", 0, 1.0, 100),
//                 Item::new("Ornithopter", 0, 8.0, 1100),
//                 Item::new("Spice Harvester", 0, 47.0, 12000),
//                 Item::new("Sietch", 0, 260.0, 130000),
//             ];
//             GameState::new(items)
//         }
//     };
//     let mut input = String::new();
//     loop {
//         game_state.update_spice();
//         println!("------------------");
//         println!("Spice: {}", game_state.get_spice());
//         println!("Clicks per second: {}", game_state.get_cps());
//         print!("Enter Command: ");
//         io::stdout().flush().unwrap();
//         input.clear();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");
//         let cmd = input.trim().to_ascii_lowercase();
//         match cmd.as_str() {
//             "" => {
//                 game_state.update_spice_by_flat(1);
//             }
//             "inventory" => {
//                 println!("--- Inventory ---");
//                 game_state.list_inventory();
//             }
//             "shop" => {
//                 println!("--- Shop ---");
//                 game_state.list_shop();
//                 print!("Enter item number to purchase or press Enter to cancel: ");
//                 io::stdout().flush().unwrap();
//                 input.clear();
//                 io::stdin()
//                     .read_line(&mut input)
//                     .expect("Failed to read line");
//                 let input = input.trim();
//                 if !input.is_empty() {
//                     let item_num = input.parse::<usize>();
//                     match item_num {
//                         Ok(i) => {
//                             if i == 0 || i > game_state.num_items() {
//                                 println!("Invalid item number");
//                                 continue;
//                             }
//                             game_state.buy_item(i - 1);
//                         }
//                         Err(_) => {
//                             println!("Invalid input");
//                         }
//                     }
//                 }
//             }
//             "exit" => {
//                 save_game(&game_state);
//                 break;
//             }
//             "save" => {
//                 save_game(&game_state);
//             }
//             _ => {
//                 println!("Unknown command");
//             }
//         }
//     }
// }

struct GameState {
    user: UserState,
}

impl GameState {
    fn new(_ctx: &mut Context) -> GameResult<Self> {
        //Check if there's a save, if not start new game
        let user = match load_game() {
            Some(state) => {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                let duration = now - state.get_time_last_updated();
                let mut mut_state = state;
                mut_state.update_spice(duration);
                mut_state.set_time_last_updated(now);
                println!("Loaded saved game.");
                mut_state
            }
            None => {
                let items = vec![
                    Item::new("Tools", 0, 0.1, 15),
                    Item::new("Fremen", 0, 1.0, 100),
                    Item::new("Ornithopter", 0, 8.0, 1100),
                    Item::new("Spice Harvester", 0, 47.0, 12000),
                    Item::new("Sietch", 0, 260.0, 130000),
                ];
                UserState::new(items)
            }
        };
        Ok(Self {user})
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f64();
        self.user.update_spice(dt);
        Ok(())
    }
    //Drawing text is based on ggez examples hello_world.rs
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb(0, 0, 0));
        let offset = 20.0;
        let text = format!("Spice: {:.2}\nClicks per second: {:.2}\n(Press ESC to exit)", self.user.get_spice(), self.user.get_cps());
        let dest_point = ggez::glam::Vec2::new(offset, offset);
        canvas.draw(graphics::Text::new(text).set_scale(48.), dest_point);
        
        
        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("spice_harvesting", "Taite Dodson, tdodson@pdx.edu")
        .default_conf(c)
        .build()
        .unwrap();

    let state = GameState::new(&mut ctx).expect("Failed to create game state");
    event::run(ctx, event_loop, state);
}
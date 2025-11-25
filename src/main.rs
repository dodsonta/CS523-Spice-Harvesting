mod item;
use item::Item;
mod clickeritem;
use clickeritem::ClickerItem;
mod userstate;
use serde_json::{to_writer, from_reader};
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::*;
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};
use userstate::UserState;

//I realize ggez has it's own save system, but I already had this implemented before I decided to use ggez
// and didn't want to figure out how to change it
fn save_game(user: &mut UserState) {
    //Convert SystemTime to seconds since epoch so we can save it with serde
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    user.set_time_last_updated(now);
    let file = File::create("savegame.json").expect("Unable to open or create file");
    to_writer(file, user).expect("Unable to write game state to file");
}

fn load_game() -> Option<UserState> {
    let file = File::open("savegame.json").ok()?;
    let user_state: UserState = from_reader(file).ok()?;
    Some(user_state)
}

//GGEZ state struct
struct GameState {
    user: UserState,
    input: String,
    shop_mode: bool,
}

impl GameState {
    //Initialize game state
    fn new(_ctx: &mut Context) -> GameResult<Self> {
        //Check if there's a save, if not start new game
        let user = match load_game() {
            Some(state) => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64();
                //Load offline progress
                let duration = now - state.get_time_last_updated();
                let mut mut_state = state;
                mut_state.update_spice(duration);
                mut_state.set_time_last_updated(now);
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
                let clicker_items = vec![
                    ClickerItem::new("CHOAM Charter", 2.0, 100),
                    ClickerItem::new("Guild Satellite", 3.0, 500),
                ];
                UserState::new(items, clicker_items)
            }
        };
        Ok(Self {
            user,
            input: String::new(),
            shop_mode: false,
        })
    }
}

//GGEZ event handler implementation
impl ggez::event::EventHandler for GameState {
    //Per-frame update
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f64();
        //Update spice every frame based on sps
        self.user.update_spice(dt);
        Ok(())
    }
    //Drawing text is based on ggez examples hello_world.rs
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //Create all black canvas
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb(0, 0, 0));
        let offset = 32.0;
        let top_text_size = 48.0;
        let normal_text_size = 32.0;

        //Drawing the spice and sps info
        let spice_text = format!(
            "Spice: {:.2}\nSpice per second: {:.2}\nSpice per click: {:.2}",
            self.user.get_spice(),
            self.user.get_sps(),
            self.user.get_spc()
        );
        //Drawing spice info at top left corner
        let spice_pos = ggez::glam::Vec2::new(offset, offset);
        //Text set a little bigger since it's important info
        canvas.draw(
            graphics::Text::new(spice_text).set_scale(top_text_size),
            spice_pos,
        );

        //Drawing the shop/inventory info
        let middle_y = offset + (top_text_size * 3.0) + 8.0; //Sets text to be a little under the spice info
        let middle_pos = ggez::glam::Vec2::new(offset, middle_y);
        //Draw either shop or inventory based on mode
        if self.shop_mode {
            let shop_text = self.user.list_shop();
            canvas.draw(
                graphics::Text::new(shop_text).set_scale(normal_text_size),
                middle_pos,
            );
        } else {
            let inventory_text = self.user.list_inventory();
            canvas.draw(
                graphics::Text::new(inventory_text).set_scale(normal_text_size),
                middle_pos,
            );
        }

        //Draw command prompt
        let (_w, h) = ctx.gfx.drawable_size(); //Get height of game screen
        let bot_y = h - offset - normal_text_size; //Y one line up from bottom
        let bot_pos = ggez::glam::Vec2::new(offset, bot_y);
        //Draw different input prompt text based on mode
        if self.shop_mode {
            let input_text = format!("Purchase Item Number: {}", self.input);
            canvas.draw(graphics::Text::new(input_text).set_scale(32.), bot_pos);
        } else {
            let input_text = format!("Enter Command: {}", self.input);
            canvas.draw(graphics::Text::new(input_text).set_scale(32.), bot_pos);
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn text_input_event(&mut self, _ctx: &mut Context, character: char) -> GameResult {
        // Ignore control characters like backspace, enter, etc. else add to input
        if character.is_control() {
            Ok(())
        } else {
            self.input.push(character);
            Ok(())
        }
    }

    //Using this doc.rs as an example: https://docs.rs/ggez/latest/ggez/input/keyboard/index.html
    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        //Using matching to handle different keys
        match input.keycode {
            //If escape, save and quit
            Some(KeyCode::Escape) => {
                save_game(&mut self.user);
                println!("Game saved. Exiting...");
                ctx.request_quit();
            }
            //If enter, process command
            Some(KeyCode::Return) => {
                let cmd = self.input.trim().to_ascii_lowercase();
                //Shop mode commands
                if self.shop_mode {
                    //If in shop mode try to get input number, if not valid exit shop
                    let item_num = cmd.parse::<usize>();
                    match item_num {
                        Ok(i) => {
                            let total_items = self.user.total_num_items();
                            if i == 0 || i > total_items {
                                //If invalid item number
                                println!("Invalid item number");
                            } else if i > self.user.num_items() {
                                //If it's a clicker item (Items are listed first)
                                self.user.buy_clicker_item(i - self.user.num_items() - 1);
                            } else {
                                //It's a normal item
                                self.user.buy_item(i - 1);
                            }
                        }
                        Err(_) => {
                            println!("Exited shop.");
                        }
                    }
                    self.shop_mode = false;

                //Normal mode commands
                //If empty input, treat as click (on the off chance the user has no mouse)
                } else if cmd.is_empty() {
                    self.user.update_spice_by_click();
                //If "save" command, save game
                } else if cmd == "save" {
                    save_game(&mut self.user);
                    println!("Game saved.");
                // If "exit" command, save and quit
                } else if cmd == "exit" {
                    save_game(&mut self.user);
                    ctx.request_quit();
                //If "shop" command, enter shop mode
                } else if cmd == "shop" {
                    self.shop_mode = true;
                //Else give an error message
                } else {
                    println!("Unknown command");
                }
                self.input.clear();
            }
            //If backspace, remove last character
            Some(KeyCode::Back) => {
                self.input.pop();
            }
            _ => {}
        }
        Ok(())
    }

    //Clicking increases spice by 1
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        self.user.update_spice_by_click();
        Ok(())
    }
}

pub fn main() {
    //Set screen title to "Spice Harvesting"
    let screen_setup = ggez::conf::WindowSetup::default().title("Spice Harvesting");
    //Set window size to 1000x600
    let window_mode = ggez::conf::WindowMode::default().dimensions(1000.0, 600.0);

    //Create ggez context
    let (mut ctx, event_loop) =
        ContextBuilder::new("spice_harvesting", "Taite Dodson, tdodson@pdx.edu")
            .window_setup(screen_setup)
            .window_mode(window_mode)
            .build()
            .unwrap();
    //Create game state
    let state = GameState::new(&mut ctx).expect("Failed to create game state");
    //Run ggez event loop
    event::run(ctx, event_loop, state);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_game_save_load() {
        //Backup existing save file if it exists
        let save_path = Path::new("savegame.json");
        let old_save_path = Path::new("savegame_backup.json");
        let backed_up = if save_path.exists() {
            std::fs::rename(save_path, old_save_path)
                .expect("Failed to back up existing save file");
            true
        } else {
            false
        };

        let items = vec![
            Item::new("Tools", 1, 1.0, 10),
            Item::new("Fremen", 0, 2.0, 50),
            Item::new("Spice Harvester", 2, 10.0, 500),
        ];
        let clicker_items = vec![
            ClickerItem::new("CHOAM Charter", 2.0, 100),
            ClickerItem::new("Guild Satellite", 3.0, 500),
        ];
        let mut og_state = UserState::new(items, clicker_items);
        og_state.update_spice(100.0); //Add some spice by artificially updating 100 seconds
        save_game(&mut og_state);

        let loaded_state = load_game().expect("Failed to load game state");
        assert_eq!(loaded_state.get_spice(), og_state.get_spice());
        assert_eq!(loaded_state.get_sps(), og_state.get_sps());
        assert_eq!(loaded_state.num_items(), og_state.num_items());
        assert_eq!(
            loaded_state.num_clicker_items(),
            og_state.num_clicker_items()
        );

        //Clean up test save file
        std::fs::remove_file(save_path).expect("Failed to remove test save file");
        //Restore old save file if it was backed up
        if backed_up {
            std::fs::rename(old_save_path, save_path).expect("Failed to restore old save file");
        }
    }
}

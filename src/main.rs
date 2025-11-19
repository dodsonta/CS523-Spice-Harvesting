mod item;
use item::Item;
mod userstate;
use ggez::graphics;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::*;
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};
use userstate::UserState;

//I realize ggez has it's own save system, but I already had this implemented before I decided to use ggez
// and didn't want to figure out how to change it
fn save_game(user: &mut UserState) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    user.set_time_last_updated(now);
    let file = File::create("savegame.json").expect("Unable to open or create file");
    serde_json::to_writer(file, user).expect("Unable to write game state to file");
}

fn load_game() -> Option<UserState> {
    let file = File::open("savegame.json").ok()?;
    let user_state: UserState = serde_json::from_reader(file).ok()?;
    Some(user_state)
}

struct GameState {
    user: UserState,
    input: String,
    shop_mode: bool,
}

impl GameState {
    fn new(_ctx: &mut Context) -> GameResult<Self> {
        //Check if there's a save, if not start new game
        let user = match load_game() {
            Some(state) => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64();
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
        Ok(Self {
            user,
            input: String::new(),
            shop_mode: false,
        })
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f64();
        self.user.update_spice(dt);
        Ok(())
    }
    //Drawing text is based on ggez examples hello_world.rs
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //Create all black canvas
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb(0, 0, 0));
        let offset = 32.0;

        //Drawing the spice and cps info
        let spice_text = format!(
            "Spice: {:.2}\nClicks per second: {:.2}\n(Press ESC to exit)",
            self.user.get_spice(),
            self.user.get_cps()
        );
        let spice_dest_point = ggez::glam::Vec2::new(offset, offset);
        canvas.draw(
            graphics::Text::new(spice_text).set_scale(48.),
            spice_dest_point,
        );

        //Drawing the shop/inventory info
        if self.shop_mode {
            let shop_text = self.user.list_shop();
            let middle_y = offset + 140.0;
            let middle_pos = ggez::glam::Vec2::new(offset, middle_y);
            canvas.draw(graphics::Text::new(shop_text).set_scale(32.), middle_pos);
        } else {
            let inventory_text = self.user.list_inventory();
            let middle_y = offset + 140.0;
            let middle_pos = ggez::glam::Vec2::new(offset, middle_y);
            canvas.draw(
                graphics::Text::new(inventory_text).set_scale(32.),
                middle_pos,
            );
        }

        //Draw command prompt
        let (_w, h) = ctx.gfx.drawable_size(); //Get height of window
        if self.shop_mode {
            let input_text = format!("Purchase Item Number: {}", self.input);
            let input_dest_point = ggez::glam::Vec2::new(offset, h - offset - 48.);
            canvas.draw(
                graphics::Text::new(input_text).set_scale(32.),
                input_dest_point,
            );
        } else {
            let input_text = format!("Enter Command: {}", self.input);
            let input_dest_point = ggez::glam::Vec2::new(offset, h - offset - 48.);
            canvas.draw(
                graphics::Text::new(input_text).set_scale(32.),
                input_dest_point,
            );
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
        match input.keycode {
            Some(KeyCode::Escape) => {
                save_game(&mut self.user);
                ctx.request_quit();
            }
            Some(KeyCode::Return) => {
                let cmd = self.input.trim().to_ascii_lowercase();
                if self.shop_mode {
                    let item_num = cmd.parse::<usize>();
                    match item_num {
                        Ok(i) => {
                            if i == 0 || i > self.user.num_items() {
                                println!("Invalid item number");
                            } else {
                                self.user.buy_item(i - 1);
                            }
                        }
                        Err(_) => {
                            println!("Exited shop.");
                        }
                    }
                    self.shop_mode = false;
                } else if cmd.is_empty() {
                    self.user.update_spice_by_flat(1);
                } else if cmd == "save" {
                    save_game(&mut self.user);
                    println!("Game saved.");
                } else if cmd == "exit" {
                    save_game(&mut self.user);
                    println!("Game saved.");
                    ctx.request_quit();
                } else if cmd == "shop" {
                    self.shop_mode = true;
                } else {
                    println!("Unknown command");
                }
                self.input.clear();
            }
            Some(KeyCode::Back) => {
                self.input.pop();
            }
            _ => {}
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        self.user.update_spice_by_flat(1);
        Ok(())
    }
}

pub fn main() {
    let screen_setup = ggez::conf::WindowSetup::default().title("Spice Harvesting");
    let window_mode = ggez::conf::WindowMode::default().dimensions(1000.0, 600.0);
    let (mut ctx, event_loop) =
        ContextBuilder::new("spice_harvesting", "Taite Dodson, tdodson@pdx.edu")
            .window_setup(screen_setup)
            .window_mode(window_mode)
            .build()
            .unwrap();

    let state = GameState::new(&mut ctx).expect("Failed to create game state");
    event::run(ctx, event_loop, state);
}

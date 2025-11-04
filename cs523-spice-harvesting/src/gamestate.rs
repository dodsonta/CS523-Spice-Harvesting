use crate::item::{self, Item};
use std::time::{SystemTime};

pub struct GameState {
    //Represents the amount of spice the player has
    spice: u64,
    //Represents the items the player can purchase
    items: Vec<Item>,
}

impl GameState {
    pub fn new(items: Vec<Item>) -> GameState {
        GameState {
            spice: 0,
            items,
        }
    }

    pub fn get_spice(&self) -> u64 {
        self.spice
    }

    pub fn list_inventory(&self) {
        for item in self.items.iter() {
            if item.amt() == 0 {
                continue;
            }
            println!("{}", item.info_in_inventory());
        }
    }

    pub fn calculate_cps(&self) -> u32 {
        let mut cps = 0;
        for item in self.items.iter() {
            cps += item.amt() * item.worth();
        }
        cps
    }

    pub fn update_spice_by_flat(&mut self, increase: u64) {
        self.spice += increase;
    }

    pub fn update_spice(&mut self, initial_time: &mut SystemTime) {
        let curr_time = SystemTime::now();
        let duration = curr_time.duration_since(*initial_time).unwrap().as_secs();
        let cps: u64 = self.calculate_cps().into();
        self.spice += cps * duration;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cps(){
        let items = vec![
            Item::new("Tools", 0, 1, 10),
            Item::new("Fremen", 0, 2, 50),
            Item::new("Spice Harvester", 0, 10, 500),
        ];
        let game_state = GameState::new(items);
        let cps = game_state.calculate_cps();
        assert_eq!(cps, 21);
    }
}
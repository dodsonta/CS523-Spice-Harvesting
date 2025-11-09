use crate::item::Item;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub struct GameState {
    //Represents the amount of spice the player has
    spice: f64,
    //Represents the items the player can purchase
    items: Vec<Item>,
    cps: f64,
}

impl GameState {
    pub fn new(items: Vec<Item>) -> GameState {
        GameState {
            spice: 0.0,
            items,
            cps: 0.0,
        }
    }

    pub fn get_spice(&self) -> f64 {
        self.spice
    }

    pub fn list_inventory(&self) {
        if self.items.iter().all(|item| item.amt() == 0) {
            println!("Inventory is empty");
            return;
        }
        for item in self.items.iter() {
            if item.amt() == 0 {
                continue;
            }
            println!("{}", item.info_in_inventory());
        }
    }

    pub fn list_shop(&self) {
        let mut idx = 1;
        for item in self.items.iter() {
            println!("{}. {}", idx, item.info_in_shop());
            idx += 1;
        }
    }

    pub fn calculate_cps(&mut self) {
        let mut temp_cps = 0.0;
        for item in self.items.iter() {
            temp_cps += item.amt() as f64 * item.worth();
        }
        self.cps = temp_cps;
    }

    pub fn update_spice_by_flat(&mut self, increase: u64) {
        self.spice += increase as f64;
    }

    pub fn update_spice(&mut self, initial_time: &mut SystemTime) {
        let curr_time = SystemTime::now();
        let duration = curr_time.duration_since(*initial_time).unwrap().as_secs();
        *initial_time = curr_time;
        self.calculate_cps();
        let cps = self.cps;
        self.spice += cps * duration as f64;
    }

    pub fn num_items(&self) -> usize {
        self.items.len()
    }

    pub fn buy_item(&mut self, item_index: usize) {
        let item = &mut self.items[item_index];
        if item.cost() as f64 > self.spice {
            println!("Not enough spice to purchase {}", item.info_in_shop());
        } else {
            self.spice -= item.cost() as f64;
            item.purchase();
            println!("Purchased {}", item.info_in_shop());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cps() {
        let items = vec![
            Item::new("Tools", 1, 1.0, 10),
            Item::new("Fremen", 0, 2.0, 50),
            Item::new("Spice Harvester", 2, 10.0, 500),
        ];
        let mut game_state = GameState::new(items);
        game_state.calculate_cps();
        assert_eq!(game_state.cps, 21.0);
    }

    #[test]
    fn test_buy_item() {
        let items = vec![
            Item::new("Tools", 0, 1.0, 10),
            Item::new("Fremen", 0, 2.0, 50),
            Item::new("Spice Harvester", 0, 10.0, 500),
        ];
        let mut game_state = GameState::new(items);
        game_state.update_spice_by_flat(100);
        game_state.buy_item(0);
        assert_eq!(game_state.get_spice(), 90.0);
        assert_eq!(game_state.items[0].amt(), 1);
        assert_eq!(game_state.items[1].amt(), 0);
        assert_eq!(game_state.items[2].amt(), 0);
    }

    #[test]
    fn test_update_spice() {
        let items = vec![
            Item::new("Tools", 1, 1.0, 10),
            Item::new("Fremen", 0, 2.0, 50),
            Item::new("Spice Harvester", 0, 10.0, 500),
        ];
        let mut game_state = GameState::new(items);
        game_state.update_spice_by_flat(100);
        let mut initial_time = SystemTime::now();
        std::thread::sleep(std::time::Duration::from_secs(2));
        game_state.update_spice(&mut initial_time);
        assert!(game_state.get_spice() >= 102.0);
    }
}

use crate::item::Item;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct UserState {
    //Represents the amount of spice the player has
    spice: f64,
    //Represents the items the player can purchase
    items: Vec<Item>,
    //Current spice per second rate
    cps: f64,
    //Last time the game was updated, in seconds since epoch
    time_last_updated: f64,
}

impl UserState {
    //Initialize new user state to 0 spice, selected items, and 0 cps
    pub fn new(items: Vec<Item>) -> UserState {
        UserState {
            spice: 0.0,
            items,
            cps: 0.0,
            //Need to do it this way so I can serialize the time easily
            time_last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        }
    }

    //Gets current spice amount
    pub fn get_spice(&self) -> f64 {
        self.spice
    }

    //Gets current spice per second rate
    pub fn get_cps(&self) -> f64 {
        self.cps
    }

    //Gets last updated time
    pub fn get_time_last_updated(&self) -> f64 {
        self.time_last_updated
    }

    //Sets last updated time
    pub fn set_time_last_updated(&mut self, time: f64) {
        self.time_last_updated = time;
    }

    //Gets number of items
    pub fn num_items(&self) -> usize {
        self.items.len()
    }

    //List items in the user's inventory
    pub fn list_inventory(&self) -> String {
        let mut inventory_text = String::from("---Inventory---\n");
        //If no items, say it's empty and return
        if self.items.iter().all(|item| item.get_amt() == 0) {
            inventory_text.push_str("Inventory is empty\n");
            return inventory_text;
        }

        //Iterate through items, listing any the user owns at least 1 of
        for item in self.items.iter() {
            if item.get_amt() == 0 {
                continue;
            }
            inventory_text.push_str(&format!("{}\n", item.info_in_inventory()));
        }
        inventory_text
    }

    //List items available in the shop
    pub fn list_shop(&self) -> String {
        let mut shop_text = String::from("---Shop---\n");
        let mut idx = 1;
        for item in self.items.iter() {
            shop_text.push_str(&format!("{}. {}\n", idx, item.info_in_shop()));
            idx += 1;
        }
        shop_text
    }

    //Calculates cps based on owned items
    pub fn calculate_cps(&mut self) {
        let mut temp_cps = 0.0;
        for item in self.items.iter() {
            temp_cps += item.get_amt() as f64 * item.get_worth();
        }
        self.cps = temp_cps;
        //Rounding to 2 decimal places since getting very long floats otherwise
        self.cps = (self.cps * 100.0).round() / 100.0;
    }

    //Updates spice by a flat amount (used for clicks)
    pub fn update_spice_by_flat(&mut self, increase: u64) {
        self.spice += increase as f64;
    }

    //Updates spice based on cps and time difference
    pub fn update_spice(&mut self, dt: f64) {
        if !dt.is_finite() || dt <= 0.0 {
            return;
        }
        self.calculate_cps();
        self.spice += self.cps * dt;
    }

    //Attempts to buy an item from the shop
    pub fn buy_item(&mut self, item_index: usize) {
        let item = &mut self.items[item_index];
        //Check if enough spice to buy the item
        if item.get_cost() as f64 > self.spice {
            println!("Not enough spice to purchase {}", item.get_name());
        } else {
            self.spice -= item.get_cost() as f64;
            item.purchase();
            println!("Purchased {}", item.get_name());
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
        let mut game_state = UserState::new(items);
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
        let mut game_state = UserState::new(items);
        game_state.update_spice_by_flat(100);
        game_state.buy_item(0);
        assert_eq!(game_state.get_spice(), 90.0);
        assert_eq!(game_state.items[0].get_amt(), 1);
        assert_eq!(game_state.items[1].get_amt(), 0);
        assert_eq!(game_state.items[2].get_amt(), 0);
    }

    #[test]
    fn test_update_spice() {
        let items = vec![
            Item::new("Tools", 1, 1.0, 10),
            Item::new("Fremen", 0, 2.0, 50),
            Item::new("Spice Harvester", 0, 10.0, 500),
        ];
        let mut game_state = UserState::new(items);
        game_state.update_spice_by_flat(100);
        game_state.update_spice(2.0);
        assert!(game_state.get_spice() >= 102.0);
    }
}

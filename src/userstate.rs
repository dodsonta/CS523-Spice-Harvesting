use crate::clickeritem::ClickerItem;
use crate::item::Item;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct UserState {
    //Represents the amount of spice the player has
    spice: f64,
    //Represents the items the player can purchase
    items: Vec<Item>,
    //Represents the clicker items the player can purchase
    clicker_items: Vec<ClickerItem>,
    //Current spice per second rate
    sps: f64,
    //Current spice per click rate
    spc: f64,
    //Last time the game was updated, in seconds since epoch
    time_last_updated: f64,
    //List of clicker items the user owns, used because clicker items are one-time purchases
    owned_clicker_items: Vec<ClickerItem>,
}

impl UserState {
    //Initialize new user state to 0 spice, selected items, and 0 sps
    pub fn new(items: Vec<Item>, clicker_items: Vec<ClickerItem>) -> UserState {
        UserState {
            spice: 0.0,
            items,
            clicker_items,
            sps: 0.0,
            spc: 1.0,
            //Need to do it this way so I can serialize the time easily
            time_last_updated: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            owned_clicker_items: vec![],
        }
    }

    //Gets current spice amount
    pub fn get_spice(&self) -> f64 {
        self.spice
    }

    //Gets current spice per second rate
    pub fn get_sps(&self) -> f64 {
        self.sps
    }

    //Gets current spice per click rate
    pub fn get_spc(&self) -> f64 {
        self.spc
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

    //Gets number of clicker items that can be purchased
    pub fn num_clicker_items(&self) -> usize {
        self.clicker_items.len()
    }

    //Gets number of all items that can be purchased
    pub fn total_num_items(&self) -> usize {
        self.items.len() + self.num_clicker_items()
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
        for clicker_item in self.owned_clicker_items.iter() {
            inventory_text.push_str(&format!("{}\n", clicker_item.info_in_inventory()));
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
        for clicker_item in self.clicker_items.iter() {
            shop_text.push_str(&format!("{}. {}\n", idx, clicker_item.info_in_shop()));
            idx += 1;
        }
        shop_text
    }

    //Calculates sps based on owned items
    pub fn calculate_sps(&mut self) {
        let mut temp_sps = 0.0;
        for item in self.items.iter() {
            temp_sps += item.get_amt() as f64 * item.get_worth();
        }
        self.sps = temp_sps;
        //Rounding to 2 decimal places since getting very long floats otherwise
        self.sps = (self.sps * 100.0).round() / 100.0;
    }

    //Update spice after a click
    pub fn update_spice_by_click(&mut self) {
        self.spice += self.spc;
    }

    //Updates spice based on sps and time difference
    pub fn update_spice(&mut self, dt: f64) {
        if !dt.is_finite() || dt <= 0.0 {
            return;
        }
        self.calculate_sps();
        self.spice += self.sps * dt;
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

    //Attempts to buy a clicker item from the shop
    pub fn buy_clicker_item(&mut self, clicker_item_index: usize) {
        let clicker_item = &mut self.clicker_items[clicker_item_index];
        //Check if enough spice to buy the clicker item
        if clicker_item.get_cost() as f64 > self.spice {
            println!("Not enough spice to purchase {}", clicker_item.get_name());
        } else {
            //Purchase the clicker item, set it to owned, and increase spc
            self.spice -= clicker_item.get_cost() as f64;
            self.spc *= clicker_item.get_multiplier();
            println!("Purchased {}", clicker_item.get_name());
            self.owned_clicker_items.push(clicker_item.clone());
            self.clicker_items.remove(clicker_item_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sps() {
        let items = vec![
            Item::new("Tools", 1, 1.0, 10),
            Item::new("Fremen", 0, 2.0, 50),
            Item::new("Spice Harvester", 2, 10.0, 500),
        ];
        let clicker_items = vec![];
        let mut game_state = UserState::new(items, clicker_items);
        game_state.calculate_sps();
        assert_eq!(game_state.sps, 21.0);
    }

    #[test]
    fn test_buy_item() {
        let items = vec![
            Item::new("Tools", 0, 1.0, 10),
            Item::new("Fremen", 0, 2.0, 50),
            Item::new("Spice Harvester", 0, 10.0, 500),
        ];
        let clicker_items = vec![];
        let mut game_state = UserState::new(items, clicker_items);
        for _ in 0..100 {
            game_state.update_spice_by_click();
        }
        game_state.buy_item(0);
        assert_eq!(game_state.get_spice(), 90.0);
        assert_eq!(game_state.items[0].get_amt(), 1);
        assert_eq!(game_state.items[1].get_amt(), 0);
        assert_eq!(game_state.items[2].get_amt(), 0);
    }

    #[test]
    fn test_update_spice_sps() {
        let items = vec![
            Item::new("Tools", 1, 1.0, 10),
            Item::new("Fremen", 0, 2.0, 50),
            Item::new("Spice Harvester", 0, 10.0, 500),
        ];
        let clicker_items = vec![];
        let mut game_state = UserState::new(items, clicker_items);
        for _ in 0..100 {
            game_state.update_spice_by_click();
        }
        game_state.update_spice(2.0);
        assert!(game_state.get_spice() >= 102.0);
    }

    #[test]
    fn test_buy_clicker_item() {
        let items = vec![];
        let clicker_items = vec![
            ClickerItem::new("Test Item 1", 2.0, 100),
            ClickerItem::new("Test Item 2", 3.0, 500),
        ];
        let mut game_state = UserState::new(items, clicker_items);
        for _ in 0..200 {
            game_state.update_spice_by_click();
        }
        game_state.buy_clicker_item(0);
        assert_eq!(game_state.get_spice(), 100.0);
        assert_eq!(game_state.spc, 2.0);
        assert_eq!(game_state.clicker_items.len(), 1);
        assert_eq!(game_state.owned_clicker_items.len(), 1);
    }

    #[test]
    fn test_time_update() {
        let items = vec![];
        let clicker_items = vec![];
        let mut game_state = UserState::new(items, clicker_items);
        let initial_time = game_state.get_time_last_updated();
        game_state.set_time_last_updated(initial_time + 1000.0);
        assert_eq!(game_state.get_time_last_updated(), initial_time + 1000.0);
    }
}

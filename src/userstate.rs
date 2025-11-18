use crate::item::Item;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct UserState {
    //Represents the amount of spice the player has
    spice: f64,
    //Represents the items the player can purchase
    items: Vec<Item>,
    cps: f64,
    time_last_updated: f64,
}

impl UserState {
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

    pub fn get_spice(&self) -> f64 {
        self.spice
    }

    pub fn get_cps(&self) -> f64 {
        self.cps
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
        //Rounding to 2 decimal places since getting very long floats otherwise
        self.cps = (self.cps * 100.0).round() / 100.0;
    }

    pub fn update_spice_by_flat(&mut self, increase: u64) {
        self.spice += increase as f64;
    }

    pub fn update_spice(&mut self, dt: f64) {
        if !dt.is_finite() || dt <= 0.0 {
            return;
        }
        self.calculate_cps();
        self.spice += self.cps * dt;
    }

    // pub fn update_spice(&mut self) {
    //     let curr_time = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .unwrap()
    //         .as_secs_f64();
    //     let duration = curr_time - self.time_last_updated;
    //     self.time_last_updated = curr_time;
    //     self.update_spice_dt(duration);

    //     //Rounding to 1 decimal places since getting very long floats otherwise
    //     //self.spice = (self.spice * 10.0).round() / 10.0;
    // }

    pub fn get_time_last_updated(&self) -> f64 {
        self.time_last_updated
    }

    pub fn set_time_last_updated(&mut self, time: f64) {
        self.time_last_updated = time;
    }

    pub fn num_items(&self) -> usize {
        self.items.len()
    }

    pub fn buy_item(&mut self, item_index: usize) {
        let item = &mut self.items[item_index];
        if item.cost() as f64 > self.spice {
            println!("Not enough spice to purchase {}", item.name());
        } else {
            self.spice -= item.cost() as f64;
            item.purchase();
            println!("Purchased {}", item.name());
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
        let mut game_state = UserState::new(items);
        game_state.update_spice_by_flat(100);
        game_state.update_spice(2.0);
        assert!(game_state.get_spice() >= 102.0);
    }
}

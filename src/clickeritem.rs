use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ClickerItem {
    name: String,
    owned: bool,
    multiplier: f64,
    cost: u32,
}

impl ClickerItem {
    //Create new clicker item (all must be hardcoded)
    pub fn new(name: &str, owned: bool, multiplier: f64, cost: u32) -> ClickerItem {
        ClickerItem {
            name: name.to_string(),
            owned,
            multiplier,
            cost,
        }
    }

    //Get name of the clicker item
    pub fn get_name(&self) -> &str {
        &self.name
    }

    //Get amount of the clicker item owned
    pub fn get_owned(&self) -> bool {
        self.owned
    }

    //Get multiplier of the clicker item
    pub fn get_multiplier(&self) -> f64 {
        self.multiplier
    }

    //Get cost of the clicker item
    pub fn get_cost(&self) -> u32 {
        self.cost
    }

    //Set owned status
    pub fn set_owned(&mut self, owned: bool) {
        self.owned = owned;
    }

    //Get a string with clicker item info for inventory display
    pub fn info_in_inventory(&self) -> String {
        format!(
            "{}: Click Multiplier: {}",
            self.name, self.multiplier
        )
    }

    //Get a string with clicker item info for shop display
    pub fn info_in_shop(&self) -> String {
        format!(
            "{}: Cost: {}, Click Multiplier: {}",
            self.name, self.cost, self.multiplier
        )
    }
    
}
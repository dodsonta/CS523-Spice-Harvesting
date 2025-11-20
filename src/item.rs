use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    amt: u32,
    worth: f64,
    cost: u32,
}

impl Item {
    //Create new item (all must be hardcoded)
    pub fn new(name: &str, amt: u32, worth: f64, cost: u32) -> Item {
        Item {
            name: name.to_string(),
            amt,
            worth,
            cost,
        }
    }

    //Get name of the item
    pub fn get_name(&self) -> &str {
        &self.name
    }

    //Get amount of the item owned
    pub fn get_amt(&self) -> u32 {
        self.amt
    }

    //Get worth (cps) of the item
    pub fn get_worth(&self) -> f64 {
        self.worth
    }

    //Get cost of the item
    pub fn get_cost(&self) -> u32 {
        self.cost
    }

    //Get a string with item info for inventory display
    pub fn info_in_inventory(&self) -> String {
        format!(
            "{}: Amount Owned: {}, Clicks per second: {}",
            self.name, self.amt, self.worth
        )
    }

    //Get a string with item info for shop display
    pub fn info_in_shop(&self) -> String {
        format!(
            "{}: Cost: {}, Clicks per second: {}",
            self.name, self.cost, self.worth
        )
    }

    //Increases the amount owned by 1 and increases the cost by 15%
    pub fn purchase(&mut self) {
        self.amt += 1;
        self.cost = (self.cost as f64 * 1.15).ceil() as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let test_item = Item::new("TestItem", 5, 10.0, 2);
        assert_eq!(test_item.get_amt(), 5);
        assert_eq!(test_item.get_worth(), 10.0);
        assert_eq!(test_item.get_cost(), 2);
    }

    #[test]
    fn test_info_in_inventory() {
        let test_item = Item::new("TestItem", 2, 10.0, 2);
        let info = test_item.info_in_inventory();
        assert_eq!(info, "TestItem: Amount Owned: 2, Clicks per second: 10");
    }

    #[test]
    fn test_info_in_shop() {
        let test_item = Item::new("TestItem", 2, 10.0, 2);
        let info = test_item.info_in_shop();
        assert_eq!(info, "TestItem: Cost: 2, Clicks per second: 10");
    }

    #[test]
    fn test_purchase() {
        let mut test_item = Item::new("TestItem", 2, 10.0, 2);
        test_item.purchase();
        assert_eq!(test_item.get_amt(), 3);
        assert_eq!(test_item.get_cost(), 3); // 2 * 1.15.ceil() = 3
    }
}

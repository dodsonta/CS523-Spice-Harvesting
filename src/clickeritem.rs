use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ClickerItem {
    name: String,
    multiplier: f64,
    cost: u32,
}

impl ClickerItem {
    //Create new clicker item (all must be hardcoded)
    pub fn new(name: &str, multiplier: f64, cost: u32) -> ClickerItem {
        ClickerItem {
            name: name.to_string(),
            multiplier,
            cost,
        }
    }

    //Get name of the clicker item
    pub fn get_name(&self) -> &str {
        &self.name
    }

    //Get multiplier of the clicker item
    pub fn get_multiplier(&self) -> f64 {
        self.multiplier
    }

    //Get cost of the clicker item
    pub fn get_cost(&self) -> u32 {
        self.cost
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clicker_item_creation() {
        let clicker_item = ClickerItem::new("Test Clicker", 2.0, 100);
        assert_eq!(clicker_item.get_name(), "Test Clicker");
        assert_eq!(clicker_item.get_multiplier(), 2.0);
        assert_eq!(clicker_item.get_cost(), 100);
    }

    #[test]
    fn test_clicker_item_info() {
        let clicker_item = ClickerItem::new("Test Clicker", 2.0, 100);
        let inventory_info = clicker_item.info_in_inventory();
        let shop_info = clicker_item.info_in_shop();
        assert_eq!(inventory_info, "Test Clicker: Click Multiplier: 2");
        assert_eq!(shop_info, "Test Clicker: Cost: 100, Click Multiplier: 2");
    }
}
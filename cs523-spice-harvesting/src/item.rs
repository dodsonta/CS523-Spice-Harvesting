pub struct Item {
    name: String,
    amt: u32,
    worth: u32,
    cost: u32,
}

impl Item {
    pub fn new(name: &str, amt: u32, worth: u32, cost: u32) -> Item {
        Item {
            name: name.to_string(),
            amt,
            worth,
            cost,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn amt(&self) -> u32 {
        self.amt
    }
    pub fn worth(&self) -> u32 {
        self.worth
    }
    pub fn cost(&self) -> u32 {
        self.cost
    }
    pub fn info_in_inventory(&self) -> String {
        format!(
            "{}: Amount Owned: {}, Clicks per second: {}",
            self.name, self.amt, self.worth
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let test_item = Item::new("TestItem", 5, 10, 2);
        assert_eq!(test_item.name(), "TestItem");
        assert_eq!(test_item.amt(), 5);
        assert_eq!(test_item.worth(), 10);
        assert_eq!(test_item.cost(), 2);
    }

    #[test]
    fn test_info_in_inventory() {
        let test_item = Item::new("TestItem", 2, 10, 2);
        let info = test_item.info_in_inventory();
        assert_eq!(info, "TestItem: Amount Owned: 2, Clicks per second: 10");
    }
}

mod item;
use item::Item;

fn calculate_cps (items: &[Item]) -> u32 {
    let mut cps = 0;
    for item in items {
        cps += item.amt() * item.worth();
    }
    cps
}
fn main() {
    // let items = [
    //     Item::new("Tools", 0, 1, 10),
    //     Item::new("Fremen", 0, 2, 50),
    //     Item::new("Spice Harvester", 0, 10, 500),
    // ];
    // let mut spice = 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cps(){
        let items = [
        Item::new("Tools", 1, 1, 10),
        Item::new("Fremen", 0, 2, 50),
        Item::new("Spice Harvester", 2, 10, 500),
        ];
        let cps = calculate_cps(&items);
        assert_eq!(cps, 21);
    }
}

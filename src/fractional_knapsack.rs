use std::cmp::Ordering;

#[derive(Debug)]
pub struct Item {
    id: String,
    weight: f32,
    value: u32,
}

impl Item {
    pub fn new(id: &str, weight: f32, value: u32) -> Item {
        let id = id.to_string(); 

        Item {
            id,
            weight,
            value,
        }
    }
}

impl Eq for Item {}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.value as f32 / self.weight == other.value as f32 / other.weight
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        let ratio_self = self.value as f32 / self.weight;
        let ratio_other = other.value as f32 / other.weight;

        if ratio_self < ratio_other {
            Ordering::Less
        } else if ratio_self > ratio_other {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

pub fn fractional_knapsack(capacity: u32, items: &mut [Item]) -> Vec<Item> {
    items.sort();
    items.reverse();

    let mut solution = Vec::new();
    let mut capacity_left = capacity as f32;

    for item in items {
        if capacity_left == 0.0 {
            break
        }
        let fraction = if item.weight < capacity_left { 1.0 } else { (capacity_left as f32) / (item.weight as f32) };
        let added_item = Item {
            id: item.id.to_string(),
            weight: fraction * item.weight,
            value: item.value,
        };
        solution.push(added_item);

        capacity_left -= fraction * item.weight;
    }

    solution
}

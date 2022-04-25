use std::cmp::max;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Item {
    weight: u32,
    value: u32,
}

impl Item {
    pub fn new(weight: u32, value: u32) -> Item {
        Item { weight, value }
    }
}

impl Eq for Item {}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.value / self.weight == other.value / other.weight
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        let ratio_self = self.value / self.weight;
        let ratio_other = other.value / other.weight;

        if ratio_self < ratio_other {
            Ordering::Less
        } else if ratio_self > ratio_other {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

pub fn _01_knapsack(capacity: usize, items: &[Item]) -> () {
    let n = items.len();
    let mut matrix = vec![vec![0; (capacity + 1) as usize]; n + 1];

    for i in 1..=n {
        for j in 1..=capacity {
            let item = &items[i - 1];
            if item.weight > j as u32 {
                matrix[i][j] = matrix[i - 1][j];
            } else {
                matrix[i][j] = max(
                    matrix[i - 1][j],
                    item.value + matrix[i - 1][j - item.weight as usize],
                );
            }
        }
    }

    // printing
    for row in matrix {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
}

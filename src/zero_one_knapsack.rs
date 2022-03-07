// TODO
use std::cmp::Ordering;
use std::cmp::max;

#[derive(Debug)]
pub struct Item {
    id: String,
    weight: u32,
    value: u32,
}

impl Item {
    pub fn new(id: &str, weight: u32, value: u32) -> Item {
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
        self.value as f32 / self.weight as f32 == other.value as f32 / other.weight as f32
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

pub fn zero_one_knapsack(capacity: u32, items: &mut [Item]) {
    items.sort();

    let n = items.len();
    let mut t = vec![vec![0; (capacity + 1) as usize]; n + 2];

    // fill the table top to bottom, left to right
    for i in 1..n+2 {
        let value = items[i-1].value;
        let weight = items[i-1].weight as i32;

        for j in 1..n+2 {

            let t0 = t[i-1][j];
            let t1;

            let test = j as i32 - weight;
            if test < 0 {
                t1 = 0;
            } else {
                t1 = value + t[i-1][test as usize];
            }

            t[i][j] = max(t0, t1);
        }
    }


    println!("{:?}", t);
}

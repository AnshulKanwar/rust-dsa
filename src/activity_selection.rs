// TODO
// Write Tests

use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
pub struct Activity {
    pub start_time: u32,
    pub finish_time: u32,
}

impl Activity {
    pub fn new(start_time: u32, finish_time: u32) -> Activity {
        Activity {
            start_time,
            finish_time,
        }
    }
}

impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Activity {{ start_time: {}, finish_time: {} }}",
            self.start_time, self.finish_time
        )
    }
}

impl Eq for Activity {}

impl PartialEq for Activity {
    fn eq(&self, other: &Self) -> bool {
        self.finish_time == other.finish_time
    }
}

impl PartialOrd for Activity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Activity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.finish_time.cmp(&other.finish_time)
    }
}

// TODO
// Why are we sending a mutable reference tho.
// Refactor so it can be done without a mutable reference
pub fn activity_selection(activities: &mut [Activity]) -> Vec<&Activity> {
    activities.sort();

    let mut solution = Vec::new();
    solution.push(&activities[0]);

    for activity in activities[1..].iter() {
        if activity.start_time >= solution.last().unwrap().finish_time {
            solution.push(&activity);
        }
    }

    solution
}

// TODO
// write tests

use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Job {
    pub id: String,
    deadline: usize,
    profit: u32,
}

impl Job {
    pub fn new(id: &str, deadline: usize, profit: u32) -> Job {
        let id = id.to_string();
        Job {
            id,
            deadline,
            profit,
        }
    }
}

impl Eq for Job {}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.profit == other.profit
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        self.profit.cmp(&other.profit)
    }
}

pub fn job_sequencing(jobs: &mut [Job]) -> Vec<Option<&Job>> {
    // sort jobs
    jobs.sort();
    jobs.reverse();

    let mut solution = vec![None; jobs.len()];

    for job in jobs.iter() {
        let mut index = job.deadline - 1;
        while index > 0 && solution[index] != None {
            index -= 1;
        }

        if solution[index] == None {
            solution[index] = Some(job);
        }
    }

    solution
}

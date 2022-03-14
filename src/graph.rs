#[warn(dead_code)]
use std::collections::LinkedList;

#[derive(Debug)]
pub struct Graph {
    n_nodes: usize,
    adj_list: Vec<LinkedList<usize>>,
}

impl Graph {
    pub fn new(n_nodes: usize) -> Graph {
        Graph {
            n_nodes,
            adj_list: vec![LinkedList::new(); n_nodes],
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.adj_list[from].push_back(to);
    }
    fn dfs(&self, v: usize, visited: &mut Vec<bool>) {
        visited[v] = true;

        for i in &self.adj_list[v] {
            if visited[*i] == true {
                continue;
            }
            self.dfs(*i, visited);
        }
    }
    fn is_connected(&self) -> bool {
        let mut visited = vec![false; self.n_nodes];
        self.dfs(0, &mut visited);

        visited.iter().all(|&i| i)
    }
    pub fn is_eularian(&self) -> String {
        if !self.is_connected() {
            return String::from("Graph is not Eularian");
        }

        // Count vertices with odd degree
        let mut odd_count = 0;
        for vertex in &self.adj_list {
            if vertex.len() % 2 != 0 {
                odd_count += 1;
            }
        }

        if odd_count == 0 {
            return String::from("Graph is Eularian");
        } else if odd_count == 2 {
            return String::from("Graph is Semi Eularian");
        } else if odd_count > 2 {
            return String::from("Graph is not Eularian");
        } else {
            panic!("odd_count is {}", odd_count);
        }
    }
    pub fn fleurys_algorithm(&self) -> Option<Vec<usize>> {
        unimplemented!("Fleurys Algorithm cannot be implemented because rust doesn't support removing items from linked list.")
        // // if disconnected graph then return None
        // if !self.is_connected() {
        //     return None;
        // }

        // // start with vertex of odd degree
        // // if no odd degree vertex then take any random vertex
        // let mut starting_vertex = 0;

        // for vertex in 1..self.adj_list.len() {
        //     if self.adj_list[vertex].len() % 2 != 0 {
        //         starting_vertex = vertex;
        //         break;
        //     }
        // }

        // let graph = self.clone();
        // for i in &self.adj_list[starting_vertex] {

        // }

        // println!("{:?}", starting_vertex);

        // None
    }
    pub fn topological_sort(&self, order: &mut Vec<usize>) {
        // Find in degrees
        let mut in_degree: Vec<usize> = vec![0; self.n_nodes];
        for i in &self.adj_list {
            for j in i {
                in_degree[*j] += 1;
            }
        }

        let mut q = Vec::new();
        // Add nodes with 0 degree in queue
        for i in 0..in_degree.len() {
            if in_degree[i] == 0 {
                q.push(i);
            }
        }

        loop {
            if q.len() == 0 {
                break;
            }

            order.push(q.remove(0));

            // Decrease the indegree of all affected nodes
            for j in &self.adj_list[*order.last().unwrap()] {
                in_degree[*j] -= 1;
                if in_degree[*j] == 0 {
                    q.push(*j);
                }
            }
        }
        println!("{:?}", order);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connected_graph() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 0);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert!(graph.is_connected());
    }

    #[test]
    fn disconnected_graph() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 0);

        assert!(!graph.is_connected());
    }

    #[test]
    fn eularian_graph() {
        let mut graph = Graph::new(6);
        graph.add_edge(0, 1);
        graph.add_edge(0, 5);
        graph.add_edge(1, 0);
        graph.add_edge(1, 2);
        graph.add_edge(2, 1);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(2, 5);
        graph.add_edge(3, 2);
        graph.add_edge(3, 4);
        graph.add_edge(4, 2);
        graph.add_edge(4, 3);
        graph.add_edge(5, 0);
        graph.add_edge(5, 2);

        assert_eq!(graph.is_eularian(), "Graph is Eularian");
    }

    #[test]
    fn semi_eularian_graph() {
        let mut graph = Graph::new(8);
        graph.add_edge(0, 1);
        graph.add_edge(0, 3);
        graph.add_edge(1, 0);
        graph.add_edge(1, 2);
        graph.add_edge(2, 1);
        graph.add_edge(2, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 0);
        graph.add_edge(3, 2);
        graph.add_edge(4, 2);
        graph.add_edge(4, 5);
        graph.add_edge(5, 4);
        graph.add_edge(5, 6);
        graph.add_edge(5, 7);
        graph.add_edge(6, 5);
        graph.add_edge(6, 7);
        graph.add_edge(7, 5);
        graph.add_edge(7, 6);

        assert_eq!(graph.is_eularian(), "Graph is Semi Eularian");
    }

    #[test]
    fn non_eularian_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(0, 3);
        graph.add_edge(1, 0);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 0);
        graph.add_edge(2, 1);
        graph.add_edge(3, 0);
        graph.add_edge(3, 1);
        graph.add_edge(3, 4);
        graph.add_edge(4, 3);

        assert_eq!(graph.is_eularian(), "Graph is not Eularian");
    }
}

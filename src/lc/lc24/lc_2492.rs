// https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities/
// 2492. Minimum Score of a Path Between Two Cities
pub struct Solution;
use std::collections::HashMap;
struct Node {
    min_dist: i32,
    roads: Vec<i32>,
    visited: bool,
}
impl Default for Node {
    fn default() -> Self {
        Node {
            min_dist: i32::MAX,
            roads: Vec::new(),
            visited: false,
        }
    }
}
impl Node {
    fn add_road(&mut self, road: i32, dist: i32) {
        if dist < self.min_dist {
            self.min_dist = dist;
        }
        self.roads.push(road);
    }
}
impl Solution {
    pub fn min_score(_n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut min_dist = i32::MAX;
        let mut graph: HashMap<i32, Node> = HashMap::new();
        for road in roads {
            graph.entry(road[0]).or_default().add_road(road[1], road[2]);
            graph.entry(road[1]).or_default().add_road(road[0], road[2]);
        }
        let mut visit_node = vec![1];
        while let Some(node_idx) = visit_node.pop() {
            if let Some(node) = graph.get_mut(&node_idx) {
                if node.visited {
                    continue;
                }
                node.visited = true;
                if node.min_dist < min_dist {
                    min_dist = node.min_dist;
                }
                for n_node_idx in node.roads.iter() {
                    visit_node.push(*n_node_idx);
                }
            }
        }
        min_dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_score() {
        assert_eq!(
            Solution::min_score(
                4,
                vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]]
            ),
            5
        );
        assert_eq!(
            Solution::min_score(
                4,
                vec![vec![1, 2, 2], vec![1, 3, 4], vec![3, 4, 7], vec![1, 4, 7]]
            ),
            2
        );
    }
}

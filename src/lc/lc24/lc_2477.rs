// https://leetcode.com/problems/minimum-fuel-cost-to-report-to-the-capital/
// 2477. Minimum Fuel Cost to Report to the Capital
pub struct Solution;
impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let seats = seats as i64;
        let mut q = vec![];
        let mut cost = vec![(0, 0); roads.len() + 1];
        let mut next_nodes = vec![vec![]; roads.len() + 1];
        for road in roads {
            next_nodes[road[0] as usize].push(road[1] as usize);
            next_nodes[road[1] as usize].push(road[0] as usize);
        }
        q.push((usize::MAX, 0, 0));
        while let Some((parent, node, visited)) = q.pop() {
            if visited == 0 {
                q.push((parent, node, 1));
                for &n in &next_nodes[node] {
                    if n != parent {
                        q.push((node, n, 0));
                    }
                }
            } else {
                let mut cost_children = 0;
                let mut children = 0;
                for &next in &next_nodes[node] {
                    if next != parent {
                        cost_children += cost[next].0 + (cost[next].1 as i64 + seats - 1) / seats;
                        children += cost[next].1;
                    }
                }
                cost[node] = (cost_children, children + 1);
            }
        }
        cost[0].0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_fuel_cost() {
        assert_eq!(Solution::minimum_fuel_cost(vec_vec![[0, 1], [0, 2], [0, 3]], 5), 3);
        assert_eq!(
            Solution::minimum_fuel_cost(vec_vec![[3, 1], [3, 2], [1, 0], [0, 4], [0, 5], [4, 6]], 2),
            7
        );
        assert_eq!(Solution::minimum_fuel_cost(vec_vec![], 1), 0);
    }
}

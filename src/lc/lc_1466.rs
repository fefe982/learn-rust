// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/
// 1466. Reorder Routes to Make All Paths Lead to the City Zero
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut visited = vec![false; n];
        let mut road: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut rev_road: HashMap<i32, Vec<i32>> = HashMap::new();
        for conn in &connections {
            road.entry(conn[0]).or_default().push(conn[1]);
            rev_road.entry(conn[1]).or_default().push(conn[0]);
        }
        let mut que: Vec<i32> = vec![0];
        let mut rev_cnt = 0;
        while let Some(node) = que.pop() {
            if visited[node as usize] {
                continue;
            }
            visited[node as usize] = true;
            if let Some(v) = rev_road.get(&node) {
                for &dst in v {
                    if !visited[dst as usize] {
                        que.push(dst);
                    }
                }
            }
            if let Some(v) = road.get(&node) {
                for &dst in v {
                    if !visited[dst as usize] {
                        rev_cnt += 1;
                        que.push(dst);
                    }
                }
            }
        }
        rev_cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_reorder() {
        assert_eq!(
            Solution::min_reorder(
                6,
                vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]]
            ),
            3
        );
        assert_eq!(
            Solution::min_reorder(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]),
            2
        );
        assert_eq!(Solution::min_reorder(3, vec![vec![1, 0], vec![2, 0]]), 0);
    }
}

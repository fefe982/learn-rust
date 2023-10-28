// https://leetcode.com/problems/shortest-path-visiting-all-nodes/
// 847. Shortest Path Visiting All Nodes
pub struct Solution;
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut min = i32::MAX;
        let m = (1 << n) - 1;
        for i in 0..n {
            let mut visited = std::collections::HashSet::<(i32, usize)>::new();
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((1 << i, i, 0));
            while let Some((mask, node, len)) = queue.pop_front() {
                if mask == m {
                    min = min.min(len);
                    println!("{}, {:?}", i, min);
                    break;
                }
                if visited.contains(&(mask, node)) {
                    continue;
                }
                visited.insert((mask, node));
                for &next in &graph[node] {
                    let nmask = mask | (1 << next);
                    if visited.contains(&(nmask, next as usize)) {
                        continue;
                    }
                    queue.push_back((nmask, next as usize, len + 1));
                }
            }
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_shortest_path_length() {
        assert_eq!(
            Solution::shortest_path_length(vec_vec![[1], [0, 2, 4], [1, 3], [2], [1, 5], [4]]),
            6
        );
        assert_eq!(
            Solution::shortest_path_length(vec_vec![[1, 2, 3], [0], [0], [0]]),
            4
        );
        assert_eq!(
            Solution::shortest_path_length(vec_vec![[1], [0, 2, 4], [1, 3, 4], [2], [1, 2]]),
            4
        );
    }
}

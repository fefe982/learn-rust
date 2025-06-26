// https://leetcode.com/problems/all-paths-from-source-to-target/
// 797. All Paths From Source to Target
pub struct Solution;
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![vec![0]];
        while !path.is_empty() {
            let mut next = vec![];
            for p in path {
                let last = *p.last().unwrap();
                if last as usize == graph.len() - 1 {
                    res.push(p);
                } else {
                    for &next_node in &graph[last as usize] {
                        let mut new_path = p.clone();
                        new_path.push(next_node);
                        next.push(new_path);
                    }
                }
            }
            path = next;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn all_paths_source_target() {
        assert_sort_eq!(
            Solution::all_paths_source_target(vec_vec![[1, 2], [3], [3], []]),
            vec_vec![[0, 1, 3], [0, 2, 3]]
        );
        assert_sort_eq!(
            Solution::all_paths_source_target(vec_vec![[4, 3, 1], [3, 2, 4], [3], [4], []]),
            vec_vec![[0, 4], [0, 3, 4], [0, 1, 3, 4], [0, 1, 2, 3, 4], [0, 1, 4]]
        );
    }
}

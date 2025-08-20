// https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes/
// 1557. Minimum Number of Vertices to Reach All Nodes
pub struct Solution;
impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut mark = vec![false; n as usize];
        for e in edges {
            mark[e[1] as usize] = true;
        }
        mark.iter()
            .enumerate()
            .filter_map(|(idx, &m)| if !m { Some(idx as i32) } else { None })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_smallest_set_of_vertices() {
        assert_eq!(
            Solution::find_smallest_set_of_vertices(
                6,
                vec_vec![[0, 1], [0, 2], [2, 5], [3, 4], [4, 2]]
            ),
            vec![0, 3]
        );
        assert_eq!(
            Solution::find_smallest_set_of_vertices(
                5,
                vec_vec![[0, 1], [2, 1], [3, 1], [1, 4], [2, 4]]
            ),
            vec![0, 2, 3]
        );
    }
}

// https://leetcode.com/problems/maximum-path-quality-of-a-graph/
// 2065. Maximum Path Quality of a Graph
pub struct Solution;
impl Solution {
    fn walk(
        g: &Vec<Vec<(usize, i32)>>,
        values: &Vec<i32>,
        cur: usize,
        max_time: i32,
        visited: &mut Vec<i32>,
        time: i32,
        mut quality: i32,
    ) -> i32 {
        visited[cur] += 1;
        if visited[cur] == 1 {
            quality += values[cur];
        }
        let mut max = 0;
        if cur == 0 {
            max = quality;
        }
        for &(nxt, t) in g[cur].iter() {
            if time + t <= max_time {
                max = max.max(Self::walk(g, values, nxt, max_time, visited, time + t, quality));
            }
        }
        visited[cur] -= 1;
        max
    }
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        let mut g = vec![vec![]; values.len()];
        for e in edges {
            g[e[0] as usize].push((e[1] as usize, e[2]));
            g[e[1] as usize].push((e[0] as usize, e[2]));
        }
        Self::walk(&g, &values, 0, max_time, &mut vec![0; values.len()], 0, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximal_path_quality() {
        assert_eq!(
            Solution::maximal_path_quality(vec![0, 32, 10, 43], vec_vec![[0, 1, 10], [1, 2, 15], [0, 3, 10]], 49),
            75
        );
        assert_eq!(
            Solution::maximal_path_quality(vec![5, 10, 15, 20], vec_vec![[0, 1, 10], [1, 2, 10], [0, 3, 10]], 30),
            25
        );
        assert_eq!(
            Solution::maximal_path_quality(
                vec![1, 2, 3, 4],
                vec_vec![[0, 1, 10], [1, 2, 11], [2, 3, 12], [1, 3, 13]],
                50
            ),
            7
        );
    }
}

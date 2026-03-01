// https://leetcode.com/problems/longest-special-path/
// 3425. Longest Special Path
pub struct Solution;
impl Solution {
    fn dfs(
        g: &Vec<Vec<(usize, i32)>>,
        nums: &Vec<i32>,
        u: usize,
        p: usize,
        depth: i32,
        path: &mut Vec<(i32, i32)>,
        cnt: &mut Vec<i32>,
        start: usize,
        max_path: &mut i32,
        min_length: &mut i32,
    ) {
        let mut s = start;
        while cnt[nums[u] as usize] == 1 {
            cnt[path[s].0 as usize] -= 1;
            s += 1;
        }
        cnt[nums[u] as usize] += 1;
        path.push((nums[u], depth));
        let d = depth - path[s].1;
        let l = (path.len() - s) as i32;
        if d > *max_path || d == *max_path && l < *min_length {
            *max_path = d;
            *min_length = l;
        }
        for &(v, w) in &g[u] {
            if v == p {
                continue;
            }
            Self::dfs(g, nums, v, u, depth + w, path, cnt, s, max_path, min_length);
        }
        path.pop();
        cnt[nums[u] as usize] -= 1;
        for i in start..s {
            cnt[path[i].0 as usize] += 1;
        }
    }
    pub fn longest_special_path(edges: Vec<Vec<i32>>, nums: Vec<i32>) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for edge in edges {
            g[edge[0] as usize].push((edge[1] as usize, edge[2]));
            g[edge[1] as usize].push((edge[0] as usize, edge[2]));
        }
        let mut max_path = 0;
        let mut min_length = i32::MAX;
        Self::dfs(
            &g,
            &nums,
            0,
            n,
            0,
            &mut vec![],
            &mut vec![0; 50001],
            0,
            &mut max_path,
            &mut min_length,
        );
        vec![max_path, min_length]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn longest_special_path() {
        assert_eq!(
            Solution::longest_special_path(vec_vec![[1, 0, 2], [0, 2, 10]], vec![2, 4, 4]),
            [10, 2]
        );
        assert_eq!(
            Solution::longest_special_path(
                vec_vec![[0, 1, 2], [1, 2, 3], [1, 3, 5], [1, 4, 4], [2, 5, 6]],
                vec![2, 1, 2, 1, 3, 1]
            ),
            [6, 2]
        );
        assert_eq!(Solution::longest_special_path(vec_vec![[1, 0, 8]], vec![2, 2]), [0, 1]);
    }
}

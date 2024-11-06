// https://leetcode.com/problems/find-minimum-diameter-after-merging-two-trees/
// 3203. Find Minimum Diameter After Merging Two Trees
pub struct Solution;
impl Solution {
    fn min_depth(edges: Vec<Vec<i32>>) -> (i32, i32) {
        let mut g = vec![vec![]; edges.len() + 1];
        let mut degree = vec![0; edges.len() + 1];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
            degree[e[0] as usize] += 1;
            degree[e[1] as usize] += 1;
        }
        let mut q = std::collections::VecDeque::new();
        let mut res = 0;
        for (i, &d) in degree.iter().enumerate() {
            if d == 1 {
                q.push_back((i, 0));
            }
        }
        while let Some((i, d)) = q.pop_front() {
            res = res.max(d);
            if degree[i] == 0 {
                return (res, res + d);
            }
            degree[i] = -1;
            for &j in &g[i] {
                if degree[j] <= 0 {
                    continue;
                }
                degree[j] -= 1;
                if degree[j] == 1 {
                    q.push_back((j, d + 1));
                } else if degree[j] == 0 {
                    res = res.max(d + 1);
                }
            }
        }
        (res, res + res)
    }
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let min1 = Self::min_depth(edges1);
        let min2 = Self::min_depth(edges2);
        (min1.0 + min2.0 + 1).max(min1.1).max(min2.1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_diameter_after_merge() {
        assert_eq!(
            Solution::minimum_diameter_after_merge(
                vec_vec![[0, 1], [2, 0], [3, 2], [3, 6], [8, 7], [4, 8], [5, 4], [3, 5], [3, 9]],
                vec_vec![[0, 1], [0, 2], [0, 3]]
            ),
            7
        );
        assert_eq!(
            Solution::minimum_diameter_after_merge(vec_vec![[0, 1], [0, 2], [0, 3]], vec_vec![[0, 1]]),
            3
        );
        assert_eq!(
            Solution::minimum_diameter_after_merge(
                vec_vec![[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]],
                vec_vec![[0, 1], [0, 2], [0, 3], [2, 4], [2, 5], [3, 6], [2, 7]]
            ),
            5
        );
    }
}

// https://leetcode.com/problems/maximize-sum-of-weights-after-edge-removals/
// 3367. Maximize Sum of Weights after Edge Removals
pub struct Solution;
impl Solution {
    fn walk(g: &Vec<Vec<(usize, i32)>>, n: usize, p: usize, k: i32) -> (i64, i64) {
        let mut val = std::collections::BinaryHeap::new();
        let mut sum = 0;
        for &(c, v) in &g[n] {
            if c == p {
                continue;
            }
            let (kn, kn1) = Self::walk(g, c, n, k);
            sum += kn;
            if kn1 + v as i64 > kn {
                val.push(std::cmp::Reverse(kn1 + v as i64 - kn));
                if val.len() > k as usize {
                    val.pop();
                }
            }
        }
        let mut min = 0;
        if val.len() == k as usize {
            min = val.pop().unwrap().0;
        }
        for v in val {
            sum += v.0;
        }
        (sum + min, sum)
    }
    pub fn maximize_sum_of_weights(edges: Vec<Vec<i32>>, k: i32) -> i64 {
        let mut g = vec![vec![]; edges.len() + 1];
        for e in edges {
            g[e[0] as usize].push((e[1] as usize, e[2]));
            g[e[1] as usize].push((e[0] as usize, e[2]));
        }
        Self::walk(&g, 0, usize::MAX, k).0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximize_sum_of_weights() {
        assert_eq!(
            Solution::maximize_sum_of_weights(vec_vec![[0, 1, 4], [0, 2, 2], [2, 3, 12], [2, 4, 6]], 2),
            22
        );
        assert_eq!(
            Solution::maximize_sum_of_weights(
                vec_vec![[0, 1, 5], [1, 2, 10], [0, 3, 15], [3, 4, 20], [3, 5, 5], [0, 6, 10]],
                3
            ),
            65
        );
    }
}

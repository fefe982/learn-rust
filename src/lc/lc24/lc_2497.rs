// https://leetcode.com/problems/maximum-star-sum-of-a-graph/
// 2497. Maximum Star Sum of a Graph
pub struct Solution;
impl Solution {
    pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        if k == 0 {
            return *vals.iter().max().unwrap();
        }
        let mut q = vec![std::collections::BinaryHeap::new(); vals.len()];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            if vals[u] > 0 {
                q[v].push(vals[u]);
            }
            if vals[v] > 0 {
                q[u].push(vals[v]);
            }
        }
        let mut ans = i32::MIN;
        for i in 0..vals.len() {
            let mut sum = vals[i];
            let mut k = k;
            while k > 0 && !q[i].is_empty() {
                sum += q[i].pop().unwrap();
                k -= 1;
            }
            ans = ans.max(sum);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_star_sum() {
        assert_eq!(
            Solution::max_star_sum(
                vec![1, 2, 3, 4, 10, -10, -20],
                vec_vec![[0, 1], [1, 2], [1, 3], [3, 4], [3, 5], [3, 6]],
                2
            ),
            16
        );
        assert_eq!(Solution::max_star_sum(vec![-5], vec_vec![], 0), -5);
    }
}

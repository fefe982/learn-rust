// https://leetcode.com/problems/k-th-nearest-obstacle-queries/
// 3275. K-th Nearest Obstacle Queries
pub struct Solution;
impl Solution {
    pub fn results_array(queries: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut h = std::collections::BinaryHeap::new();
        let mut ans = vec![];
        let k = k as usize;
        for q in queries {
            let d = q[0].abs() + q[1].abs();
            h.push(d);
            if h.len() > k {
                h.pop();
            }
            if h.len() == k {
                ans.push(*h.peek().unwrap());
            } else {
                ans.push(-1);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn results_array() {
        assert_eq!(
            Solution::results_array(vec_vec![[1, 2], [3, 4], [2, 3], [-3, 0]], 2),
            vec![-1, 7, 5, 3]
        );
        assert_eq!(
            Solution::results_array(vec_vec![[5, 5], [4, 4], [3, 3]], 1),
            vec![10, 8, 6]
        );
    }
}

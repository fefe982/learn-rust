// https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative/
// 2441. Largest Positive Integer That Exists With Its Negative
pub struct Solution;
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut pos = std::collections::BinaryHeap::new();
        let mut neg = std::collections::BinaryHeap::new();
        for n in nums {
            if n > 0 {
                pos.push(n);
            } else {
                neg.push(-n);
            }
        }
        while !pos.is_empty() && !neg.is_empty() {
            let &p = pos.peek().unwrap();
            let &n = neg.peek().unwrap();
            if p == n {
                return p;
            }
            if p > n {
                pos.pop();
            } else {
                neg.pop();
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_max_k() {
        assert_eq!(Solution::find_max_k(vec![-1, 2, -3, 3]), 3);
        assert_eq!(Solution::find_max_k(vec![-1, 10, 6, 7, -7, 1]), 7);
        assert_eq!(Solution::find_max_k(vec![-10, 8, 6, 7, -2, -3]), -1);
    }
}

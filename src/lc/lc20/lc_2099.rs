// https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/
// 2099. Find Subsequence of Length K With the Largest Sum
pub struct Solution;
impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut h = std::collections::BinaryHeap::new();
        let k = k as usize;
        for i in 0..nums.len() {
            h.push((std::cmp::Reverse(nums[i]), i));
            if h.len() > k {
                h.pop();
            }
        }
        let mut res = Vec::with_capacity(k);
        while let Some(i) = h.pop() {
            res.push(i);
        }
        res.sort_unstable_by_key(|i| i.1);
        res.into_iter().map(|i| i.0 .0).collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(nums: Vec<i32>, k: i32, expected: Vec<i32>) {
        let res = Solution::max_subsequence(nums.clone(), k);
        assert_eq!(res.len(), expected.len());
        let sum = expected.iter().sum::<i32>();
        let mut res_sum = 0;
        let mut j = 0;
        for n in res.into_iter() {
            while j < nums.len() && nums[j] != n {
                j += 1;
            }
            assert!(j < nums.len());
            j += 1;
            res_sum += n;
        }
        assert_eq!(res_sum, sum);
    }
    #[test]
    fn max_subsequence() {
        check(vec![2, 1, 3, 3], 2, vec![3, 3]);
        check(vec![-1, -2, 3, 4], 3, vec![-1, 3, 4]);
        check(vec![3, 4, 3, 3], 2, vec![4, 3]);
    }
}

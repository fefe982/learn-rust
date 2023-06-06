// https://leetcode.com/problems/number-of-distinct-averages/
// 2465. Number of Distinct Averages
pub struct Solution;
impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        let mut s = std::collections::HashSet::<i32>::new();
        nums.sort_unstable();
        let l = nums.len();
        for i in 0..l / 2 {
            s.insert(nums[i] + nums[l - i - 1]);
        }
        s.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distinct_averages() {
        assert_eq!(Solution::distinct_averages(vec![4, 1, 4, 0, 3, 5]), 2);
        assert_eq!(Solution::distinct_averages(vec![1, 100]), 1);
    }
}

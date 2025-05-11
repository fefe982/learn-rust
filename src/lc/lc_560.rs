// https://leetcode.com/problems/subarray-sum-equals-k/
// 560. Subarray Sum Equals K
pub struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut m = std::collections::HashMap::new();
        m.insert(0, 1);
        let mut sum = 0;
        let mut ans = 0;
        for num in nums {
            sum += num;
            ans += m.get(&(sum - k)).unwrap_or(&0);
            *m.entry(sum).or_insert(0) += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn subarray_sum() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
}

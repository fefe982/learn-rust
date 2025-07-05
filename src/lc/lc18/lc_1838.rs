// https://leetcode.com/problems/frequency-of-the-most-frequent-element/
// 1838. Frequency of the Most Frequent Element
pub struct Solution;
impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut max = 1;
        let mut count = 1;
        let mut last = nums[0];
        let mut diff = 0;
        for i in 1..nums.len() {
            let n = nums[i];
            diff += (n - last) * count;
            count += 1;
            while diff > k {
                diff -= n - nums[i + 1 - count as usize];
                count -= 1;
            }
            max = max.max(count);
            last = n;
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_frequency() {
        assert_eq!(Solution::max_frequency(vec![1, 2, 4], 5), 3);
        assert_eq!(Solution::max_frequency(vec![1, 4, 8, 13], 5), 2);
        assert_eq!(Solution::max_frequency(vec![3, 9, 6], 2), 1);
    }
}

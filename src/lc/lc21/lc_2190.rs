// https://leetcode.com/problems/most-frequent-number-following-key-in-an-array/
// 2190. Most Frequent Number Following Key in an Array
pub struct Solution;
impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut count = [0; 1001];
        for i in 0..nums.len() - 1 {
            if nums[i] == key {
                count[nums[i + 1] as usize] += 1;
            }
        }
        let mut ans = 0;
        for i in 1..=1000 {
            if count[i] > count[ans] {
                ans = i;
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_most_frequent() {
        assert_eq!(Solution::most_frequent(vec![1, 100, 200, 1, 100], 1), 100);
        assert_eq!(Solution::most_frequent(vec![2, 2, 2, 2, 3], 2), 2);
    }
}

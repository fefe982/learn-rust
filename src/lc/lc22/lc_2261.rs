// https://leetcode.com/problems/k-divisible-elements-subarrays/
// 2261. K-Divisible Elements Subarrays
pub struct Solution;
impl Solution {
    pub fn count_distinct(nums: Vec<i32>, k: i32, p: i32) -> i32 {
        let mut set = std::collections::HashSet::new();
        for i in 0..nums.len() {
            let mut count = 0;
            let mut v = Vec::new();
            for j in i..nums.len() {
                if nums[j] % p == 0 {
                    count += 1;
                }
                if count > k {
                    break;
                }
                v.push(nums[j]);
                set.insert(v.clone());
            }
        }
        set.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_distinct() {
        assert_eq!(Solution::count_distinct(vec![2, 3, 3, 2, 2], 2, 2), 11);
        assert_eq!(Solution::count_distinct(vec![1, 2, 3, 4], 4, 1), 10);
    }
}

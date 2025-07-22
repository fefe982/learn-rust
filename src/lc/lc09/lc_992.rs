// https://leetcode.com/problems/subarrays-with-k-different-integers/
// 992. Subarrays with K Different Integers
pub struct Solution;
impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let mut k_map = std::collections::HashMap::<i32, i32>::new();
        let mut ans = 0;
        let k = k as usize;
        let mut ls = 0;
        let mut le = 0;
        let mut k1_map = std::collections::HashMap::<i32, i32>::new();
        for i in 0..nums.len() {
            *k_map.entry(nums[i]).or_default() += 1;
            *k1_map.entry(nums[i]).or_default() += 1;
            if k_map.len() == k + 1 {
                k_map = k1_map.clone();
                ls = le;
            }
            if k_map.len() == k {
                while k1_map.len() == k {
                    let cnt = k1_map.get_mut(&nums[le]).unwrap();
                    *cnt -= 1;
                    if *cnt == 0 {
                        k1_map.remove(&nums[le]);
                    }
                    le += 1;
                }
                ans += le - ls;
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_subarrays_with_k_distinct() {
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3), 3);
    }
}

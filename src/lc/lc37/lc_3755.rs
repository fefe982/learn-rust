// https://leetcode.com/problems/find-maximum-balanced-xor-subarray-length/
// 3755. Find Maximum Balanced XOR Subarray Length
pub struct Solution;
impl Solution {
    pub fn max_balanced_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut m = std::collections::HashMap::new();
        let mut xor = 0;
        let mut diff = 0;
        m.insert((0, 0), 0);
        for i in 0..nums.len() {
            xor ^= nums[i];
            if nums[i] % 2 == 0 {
                diff += 1;
            } else {
                diff -= 1;
            }
            if let Some(&j) = m.get(&(xor, diff)) {
                ans = ans.max(i + 1 - j);
            } else {
                m.insert((xor, diff), i + 1);
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_balanced_subarray() {
        assert_eq!(Solution::max_balanced_subarray(vec![3, 1, 3, 2, 0]), 4);
        assert_eq!(Solution::max_balanced_subarray(vec![3, 2, 8, 5, 4, 14, 9, 15]), 8);
        assert_eq!(Solution::max_balanced_subarray(vec![0]), 0);
    }
}

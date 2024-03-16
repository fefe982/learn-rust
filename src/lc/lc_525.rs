// https://leetcode.com/problems/contiguous-array/
// 525. Contiguous Array
pub struct Solution;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::new();
        m.insert(0, 0);
        let mut max = 0;
        let mut s: i32 = 0;
        for (i, n) in nums.into_iter().enumerate() {
            s += if n == 0 { -1 } else { 1 };
            if let Some(&l) = m.get(&s) {
                max = max.max(i as i32 + 1 - l);
            } else {
                m.insert(s, i as i32 + 1);
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_max_length() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    }
}

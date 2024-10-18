// https://leetcode.com/problems/minimize-or-of-remaining-elements-using-operations/
// 3022. Minimize the Or Sum of Remaining Elements
pub struct Solution;
impl Solution {
    pub fn min_or_after_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let len = nums.len() as i32;
        let mask = (1 << 30) - 1;
        for i in (0..30).rev() {
            let bit = 1 << i;
            let target = ans | (bit - 1);
            let mut cnt = 0;
            let mut cur = mask;
            for &n in &nums {
                cur &= n;
                if cur | target == target {
                    cnt += 1;
                    cur = mask;
                }
            }
            if len - cnt > k {
                ans |= bit;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_or_after_operations() {
        assert_eq!(Solution::min_or_after_operations(vec![3, 5, 3, 2, 7], 2), 3);
        assert_eq!(Solution::min_or_after_operations(vec![7, 3, 15, 14, 2, 8], 4), 2);
        assert_eq!(
            Solution::min_or_after_operations(vec![10, 7, 10, 3, 9, 14, 9, 4], 1),
            15
        );
    }
}

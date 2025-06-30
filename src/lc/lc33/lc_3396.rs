// https://leetcode.com/problems/minimum-number-of-operations-to-make-elements-in-array-distinct/
// 3396. Minimum Number of Operations to Make Elements in Array Distinct
pub struct Solution;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut cnt = [0; 101];
        let mut c = 0;
        let l = nums.len() as i32;
        for n in nums.into_iter().rev() {
            if cnt[n as usize] == 0 {
                cnt[n as usize] = 1;
                c += 1;
            } else {
                break;
            }
        }
        (l - c + 2) / 3
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_operations() {
        assert_eq!(Solution::minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7]), 2);
        assert_eq!(Solution::minimum_operations(vec![4, 5, 6, 4, 4]), 2);
    }
}

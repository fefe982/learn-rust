// https://leetcode.com/problems/minimum-index-of-a-valid-split/
// 2780. Minimum Index of a Valid Split
pub struct Solution;
impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut n = 0;
        let mut c = 0;
        for &num in nums.iter() {
            if c == 0 {
                n = num;
                c = 1;
            } else if n == num {
                c += 1;
            } else {
                c -= 1;
            }
        }
        c = 0;
        for &nums in nums.iter() {
            if nums == n {
                c += 1;
            }
        }
        let mut ic = 0;
        for i in 0..nums.len() {
            if nums[i] == n {
                ic += 1;
                if ic * 2 > i + 1 && (c - ic) * 2 > (nums.len() - i - 1) {
                    return i as i32;
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_index() {
        assert_eq!(Solution::minimum_index(vec![1, 2, 2, 2]), 2);
        assert_eq!(Solution::minimum_index(vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1]), 4);
        assert_eq!(Solution::minimum_index(vec![3, 3, 3, 3, 7, 2, 2]), -1);
    }
}

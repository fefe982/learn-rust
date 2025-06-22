// https://leetcode.com/problems/sort-colors/
// 75. Sort Colors
pub struct Solution;
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut i0 = 0;
        let mut i2 = nums.len() - 1;
        while i0 < nums.len() && nums[i0] == 0 {
            i0 += 1;
        }
        if i0 >= i2 {
            return;
        }
        while i2 > 0 && nums[i2] == 2 {
            i2 -= 1;
        }
        if i2 == 0 {
            return;
        }
        let mut i = i0;
        while i <= i2 {
            match nums[i] {
                0 => {
                    nums[i] = nums[i0];
                    nums[i0] = 0;
                    while nums[i0] == 0 {
                        i0 += 1;
                    }
                    i = i.max(i0);
                }
                1 => {
                    i += 1;
                }
                2 => {
                    nums[i] = nums[i2];
                    nums[i2] = 2;
                    while nums[i2] == 2 {
                        i2 -= 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(mut nums: Vec<i32>) {
        let mut nums_clone = nums.clone();
        Solution::sort_colors(&mut nums_clone);
        nums.sort_unstable();
        assert_eq!(nums, nums_clone);
    }
    #[test]
    fn test_sort_colors() {
        check(vec![1, 0]);
        check(vec![2, 0, 2, 1, 1, 0]);
        check(vec![2, 0, 1])
    }
}

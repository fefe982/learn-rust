// https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/
// 2654. Minimum Number of Operations to Make All Array Elements Equal to 1
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let gcd = |mut x: i32, mut y: i32| -> i32 {
            loop {
                if x == 0 {
                    return y;
                }
                y %= x;
                if y == 0 {
                    return x;
                }
                x %= y;
            }
        };
        let mut g = nums[0];
        let mut c = 0;
        for i in 1..nums.len() {
            g = gcd(g, nums[i]);
            if nums[i] == 1 {
                c += 1;
            }
        }
        if g > 1 {
            return -1;
        }
        if nums[0] == 1 {
            c += 1;
        }
        if c > 0 {
            return nums.len() as i32 - c;
        }
        let mut nums = nums;
        for i in 2..=nums.len() {
            for j in (i - 1..nums.len()).rev() {
                nums[j] = gcd(nums[j], nums[j - 1]);
                if nums[j] == 1 {
                    return nums.len() as i32 + i as i32 - 2;
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
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![2, 6, 3, 4]), 4);
        assert_eq!(Solution::min_operations(vec![2, 10, 6, 14]), -1);
    }
}

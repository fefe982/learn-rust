// https://leetcode.com/problems/smallest-absent-positive-greater-than-average/
// 3678. Smallest Absent Positive Greater Than Average
pub struct Solution;
impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let sum = nums.iter().sum::<i32>();
        let mut avg = sum / nums.len() as i32;
        if avg * nums.len() as i32 <= sum {
            avg += 1;
        }
        if avg <= 0 {
            avg = 1;
        }
        for i in 0..nums.len() {
            if nums[i] > avg {
                return avg;
            } else if nums[i] == avg {
                avg += 1;
            }
        }
        avg
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_absent() {
        assert_eq!(Solution::smallest_absent(vec![-3]), 1);
        assert_eq!(Solution::smallest_absent(vec![3, 5]), 6);
        assert_eq!(Solution::smallest_absent(vec![-1, 1, 2]), 3);
        assert_eq!(Solution::smallest_absent(vec![4, -1]), 2);
    }
}

// https://leetcode.com/problems/3sum-closest/
// 16. 3Sum Closest
pub struct Solution;
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut closest = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() - 2 {
            let mut j = i + 1;
            let mut k = j + 1;
            let mut s = nums[i] + nums[j] + nums[k];
            if (s - target).abs() < (closest - target).abs() {
                closest = s;
            }
            while k + 1 < nums.len() && s < target {
                k += 1;
                s = nums[i] + nums[j] + nums[k];
                if (s - target).abs() < (closest - target).abs() {
                    closest = s;
                }
            }
            while j + 1 < k {
                while j + 1 < k && s < target {
                    j += 1;
                    s = nums[i] + nums[j] + nums[k];
                    if (s - target).abs() < (closest - target).abs() {
                        closest = s;
                    }
                }
                while j + 1 < k && s > target {
                    k -= 1;
                    s = nums[i] + nums[j] + nums[k];
                    if (s - target).abs() < (closest - target).abs() {
                        closest = s;
                    }
                }
                if closest == target {
                    return closest;
                }
            }
        }
        closest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn three_sum_closest() {
        assert_eq!(
            Solution::three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2),
            -2
        );
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }
}

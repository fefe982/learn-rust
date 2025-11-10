// https://leetcode.com/problems/trionic-array-ii/
// 3640. Trionic Array II
pub struct Solution;
impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let mut i = 0;
        let mut step = 0;
        let mut sum = 0;
        let mut max = i64::MIN;
        let mut j = 1;
        while i + 1 < nums.len() && j + 1 < nums.len() {
            if step == 0 {
                while i + 1 < nums.len() && nums[i] >= nums[i + 1] {
                    i += 1;
                }
                if i == nums.len() - 1 {
                    break;
                }
                step = 1;
            } else if step == 1 {
                sum = nums[i] as i64;
                j = i + 1;
                let mut last = nums[i];
                sum += nums[j] as i64;
                while j + 1 < nums.len() && nums[j] < nums[j + 1] {
                    if last < 0 {
                        sum -= last as i64;
                        i += 1;
                    }
                    last = nums[j];
                    j += 1;
                    sum += nums[j] as i64;
                }
                step = 2;
            } else if step == 2 {
                while j + 1 < nums.len() && nums[j] > nums[j + 1] {
                    j += 1;
                    sum += nums[j] as i64;
                }
                step = 3;
            } else if step == 3 {
                let mut si = j;
                let mut ss = nums[j] as i64;
                let mut last = 0;
                while (j + 1) < nums.len() && nums[j] < nums[j + 1] {
                    if last < 0 {
                        ss -= last as i64;
                        si += 1;
                    }
                    last = nums[j];
                    j += 1;
                    sum += nums[j] as i64;
                    max = max.max(sum);
                    ss += nums[j] as i64;
                }
                i = si;
                sum = ss;
                step = 2;
            }
            if j + 1 < nums.len() && nums[j] == nums[j + 1] {
                i = j + 1;
                j = i + 1;
                step = 0;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_sum_trionic() {
        assert_eq!(
            Solution::max_sum_trionic(vec![
                -442, 294, 839, -265, -216, -990, -142, 461, -548, 83, -191, -602, -504
            ]),
            652
        );
        assert_eq!(
            Solution::max_sum_trionic(vec![-754, 167, -172, 202, 735, -941, 992]),
            988
        );
        assert_eq!(Solution::max_sum_trionic(vec![0, -2, -1, -3, 0, 2, -1]), -4);
        assert_eq!(Solution::max_sum_trionic(vec![1, 4, 2, 7]), 14);
    }
}

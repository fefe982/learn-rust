// https://leetcode.com/problems/minimize-the-maximum-adjacent-element-difference/
// 3357. Minimize the Maximum Adjacent Element Difference
pub struct Solution;
impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut i = usize::MAX;
        let mut j = 0;
        let mut gap_max = 0;
        while j < nums.len() {
            while j < nums.len() && nums[j] == -1 {
                j += 1;
            }
            if i == usize::MAX {
                if j > 0 && j < nums.len() {
                    min = nums[j];
                    max = nums[j];
                }
            } else if j == nums.len() {
                if i != j - 1 {
                    min = min.min(nums[i]);
                    max = max.max(nums[i]);
                }
            } else if i + 1 == j {
                gap_max = gap_max.max((nums[i] - nums[j]).abs());
            } else {
                min = min.min(nums[i]).min(nums[j]);
                max = max.max(nums[i]).max(nums[j]);
            }
            i = j;
            j += 1;
        }
        if max == i32::MIN {
            return gap_max;
        }
        let min_mul = (max - min + 2) / 3 * 2;
        i = usize::MAX;
        j = 0;
        let mut res = gap_max * 2;
        while j < nums.len() {
            while j < nums.len() && nums[j] == -1 {
                j += 1;
            }
            if i == usize::MAX {
                if j > 0 && j < nums.len() {
                    res = res.max((nums[j] - min).min(max - nums[j]));
                }
            } else if j == nums.len() {
                if i != j - 1 {
                    res = res.max((nums[i] - min).min(max - nums[i]));
                }
            } else if i + 1 < j {
                let lmin = nums[i].min(nums[j]);
                let lmax = nums[i].max(nums[j]);
                let gap = if i + 2 < j { min_mul } else { i32::MAX };
                res = res.max((lmax - min).min(max - lmin).min(gap));
            }
            i = j;
            j += 1;
        }
        (res + 1) / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_difference() {
        assert_eq!(Solution::min_difference(vec![1, 12]), 11);
        assert_eq!(Solution::min_difference(vec![1, 2, -1, 10, 8]), 4);
        assert_eq!(Solution::min_difference(vec![-1, -1, -1]), 0);
        assert_eq!(Solution::min_difference(vec![-1, 10, -1, 8]), 1);
    }
}

// https://leetcode.com/problems/count-elements-with-strictly-smaller-and-greater-elements/
// 2148. Count Elements With Strictly Smaller and Greater Elements
pub struct Solution;
impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut cmin = 0;
        let mut cmax = 0;
        let len = nums.len();
        for num in nums {
            if num < min {
                min = num;
                cmin = 1;
            } else if num == min {
                cmin += 1;
            }
            if num > max {
                max = num;
                cmax = 1;
            } else if num == max {
                cmax += 1;
            }
        }
        if min == max {
            0
        } else {
            (len - cmin - cmax) as i32
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_elements() {
        assert_eq!(Solution::count_elements(vec![11, 7, 2, 15]), 2);
        assert_eq!(Solution::count_elements(vec![-3, 3, 3, 90]), 2);
        assert_eq!(Solution::count_elements(vec![1, 1, 1]), 0);
    }
}

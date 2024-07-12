// https://leetcode.com/problems/find-if-array-can-be-sorted/
// 3011. Find if Array Can Be Sorted
pub struct Solution;
impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut last_max = 0;
        let mut last_bit = 0;
        let mut cur_max = 0;
        for n in nums {
            let bit = n.count_ones();
            if bit == last_bit {
                if n < last_max {
                    return false;
                }
                cur_max = cur_max.max(n);
            } else {
                if n < cur_max {
                    return false;
                }
                last_max = cur_max;
                cur_max = n;
            }
            last_bit = bit;
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_sort_array() {
        assert_eq!(Solution::can_sort_array(vec![8, 4, 2, 30, 15]), true);
        assert_eq!(Solution::can_sort_array(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(Solution::can_sort_array(vec![3, 16, 8, 4, 2]), false);
    }
}

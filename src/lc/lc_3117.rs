// https://leetcode.com/problems/minimum-sum-of-values-by-dividing-array/
// 3117. Minimum Sum of Values by Dividing Array
pub struct Solution;
impl Solution {
    fn search(
        nums: &Vec<i32>,
        and_values: &Vec<i32>,
        i: usize,
        j: usize,
        val: i32,
        cache: &mut std::collections::HashMap<(usize, usize, i32), i32>,
    ) -> i32 {
        if i == nums.len() && j + 1 == and_values.len() && val == and_values[j] {
            return nums[i - 1];
        }
        if i >= nums.len() || j >= and_values.len() {
            return i32::MAX;
        }
        let key = (i, j, val);
        if let Some(&v) = cache.get(&key) {
            return v;
        }
        let ret = match val.cmp(&(and_values[j])) {
            std::cmp::Ordering::Less => i32::MAX,
            std::cmp::Ordering::Equal => {
                (Solution::search(nums, and_values, i, j + 1, i32::MAX, cache))
                    .min(Solution::search(nums, and_values, i + 1, j, val & nums[i], cache) - nums[i - 1])
                    + nums[i - 1]
            }
            std::cmp::Ordering::Greater => Solution::search(nums, and_values, i + 1, j, val & nums[i], cache),
        };
        cache.insert(key, ret);
        ret
    }
    pub fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        let ret = Self::search(
            &nums,
            &and_values,
            0,
            0,
            i32::MAX,
            &mut std::collections::HashMap::new(),
        );
        if ret == i32::MAX {
            -1
        } else {
            ret
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_value_sum() {
        assert_eq!(
            Solution::minimum_value_sum(
                vec![
                    32, 114, 114, 105, 61, 37, 49, 49, 122, 60, 38, 55, 114, 110, 37, 96, 62, 34, 122, 97, 98, 122, 49,
                    106, 99, 54, 57, 32, 125, 38, 36, 48, 62, 105, 41, 119, 33, 54, 125, 96, 126, 127, 124, 40, 50, 57,
                    47, 62, 97, 42, 58, 34, 119, 44, 58, 40, 60, 47, 63, 117, 35, 124, 41, 116, 53, 127, 48, 52, 33,
                    99, 98, 100, 56, 54, 61, 61, 104, 42, 110, 39, 53, 38, 101, 49, 50, 123, 40, 32, 123, 56, 115, 33,
                    63, 99, 99, 100, 61, 47, 99, 57
                ],
                vec![32, 32, 32, 32, 32, 32, 33]
            ),
            266
        );
        assert_eq!(Solution::minimum_value_sum(vec![4, 8, 9], vec![0]), 9);
        assert_eq!(Solution::minimum_value_sum(vec![1, 4, 3, 3, 2], vec![0, 3, 3, 2]), 12);
        assert_eq!(
            Solution::minimum_value_sum(vec![2, 3, 5, 7, 7, 7, 5], vec![0, 7, 5]),
            17
        );
        assert_eq!(Solution::minimum_value_sum(vec![1, 2, 3, 4], vec![2]), -1);
    }
}

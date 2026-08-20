// https://leetcode.com/problems/earliest-second-to-mark-indices-i/
// 3048. Earliest Second to Mark All Indices
pub struct Solution;
impl Solution {
    pub fn earliest_second_to_mark_indices(nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
        fn check(nums: &Vec<i32>, change_indices: &[i32]) -> bool {
            let mut last = std::collections::HashMap::new();
            for (i, &v) in change_indices.iter().enumerate() {
                last.insert(v, i);
            }
            if last.len() < nums.len() {
                return false;
            }
            let mut cnt = 0;
            for i in 0..change_indices.len() {
                if last[&change_indices[i]] == i {
                    if cnt < nums[change_indices[i] as usize - 1] {
                        return false;
                    }
                    cnt -= nums[change_indices[i] as usize - 1];
                } else {
                    cnt += 1;
                }
            }
            true
        }
        if !check(&nums, &change_indices) {
            return -1;
        }
        let mut l = 0;
        let mut r = change_indices.len();
        while l + 1 < r {
            let m = (l + r) / 2;
            if check(&nums, &change_indices[..m]) {
                r = m;
            } else {
                l = m;
            }
        }
        r as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn earliest_second_to_mark_indices() {
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![2, 2, 0], vec![2, 2, 2, 2, 3, 2, 2, 1]),
            8
        );
        assert_eq!(
            Solution::earliest_second_to_mark_indices(vec![1, 3], vec![1, 1, 1, 2, 1, 1, 1]),
            6
        );
        assert_eq!(Solution::earliest_second_to_mark_indices(vec![0, 1], vec![2, 2, 2]), -1);
    }
}

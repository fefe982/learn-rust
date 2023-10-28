// https://leetcode.com/problems/check-if-there-is-a-valid-partition-for-the-array/
// 2369. Check if There is a Valid Partition For The Array
pub struct Solution;
impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let mut v = vec![0; nums.len() + 1];
        v[0] = 1;
        for s in 0..v.len() {
            if v[s] == 0 {
                continue;
            }
            if s == nums.len() {
                return true;
            }
            if s + 1 == nums.len() {
                continue;
            }
            if s + 2 <= nums.len() {
                if nums[s] == nums[s + 1] {
                    v[s + 2] = 1;
                }
            }
            if s + 3 <= nums.len() {
                if nums[s] == nums[s + 1] && nums[s] == nums[s + 2] {
                    v[s + 3] = 1;
                }
                if nums[s] + 1 == nums[s + 1] && nums[s] + 2 == nums[s + 2] {
                    v[s + 3] = 1;
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_partition() {
        assert_eq!(
            Solution::valid_partition(vec![
                579611, 579611, 579611, 731172, 731172, 496074, 496074, 496074, 151416, 151416,
                151416
            ]),
            true
        );
        assert_eq!(Solution::valid_partition(vec![4, 4, 4, 5, 6]), true);
        assert_eq!(Solution::valid_partition(vec![1, 1, 1, 2]), false);
    }
}

// https://leetcode.com/problems/form-array-by-concatenating-subarrays-of-another-array/
// 1764. Form Array by Concatenating Subarrays of Another Array
pub struct Solution;
impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        let mut i = 0;
        for group in groups {
            'i: while i + group.len() <= nums.len() {
                for k in 0..group.len() {
                    if group[k] != nums[i + k] {
                        i += 1;
                        continue 'i;
                    }
                }
                break;
            }
            if i + group.len() > nums.len() {
                return false;
            }
            i += group.len();
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn can_choose() {
        assert_eq!(
            Solution::can_choose(vec_vec![[1, -1, -1], [3, -2, 0]], vec![1, -1, 0, 1, -1, -1, 3, -2, 0]),
            true
        );
        assert_eq!(
            Solution::can_choose(vec_vec![[10, -2], [1, 2, 3, 4]], vec![1, 2, 3, 4, 10, -2]),
            false
        );
        assert_eq!(
            Solution::can_choose(vec_vec![[1, 2, 3], [3, 4]], vec![7, 7, 1, 2, 3, 4, 7, 7]),
            false
        );
    }
}

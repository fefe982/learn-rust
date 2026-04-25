// https://leetcode.com/problems/array-with-elements-not-equal-to-average-of-neighbors/
// 1968. Array With Elements Not Equal to Average of Neighbors
pub struct Solution;
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut i = 1;
        while i + 1 < n {
            nums.swap(i, i + 1);
            i += 2;
        }
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(original: &[i32]) -> bool {
        let arranged = Solution::rearrange_array(original.to_vec());
        if original.len() != arranged.len() {
            return false;
        }

        let mut a = original.to_vec();
        let mut b = arranged.clone();
        a.sort_unstable();
        b.sort_unstable();
        if a != b {
            return false;
        }

        for i in 1..arranged.len() - 1 {
            if arranged[i - 1] + arranged[i + 1] == arranged[i] * 2 {
                return false;
            }
        }
        true
    }

    #[test]
    fn rearrange_array() {
        assert!(is_valid(&[1, 2, 3, 4, 5]));
        assert!(is_valid(&[6, 2, 0, 9, 7]));
    }
}

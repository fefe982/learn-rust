// https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away/
// 1437. Check If All 1's Are at Least Length K Places Away
pub struct Solution;
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut l = k;
        for i in nums {
            if i == 1 {
                if l < k {
                    return false;
                }
                l = 0;
            } else {
                l += 1;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn k_length_apart() {
        assert_eq!(Solution::k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2), true);
        assert_eq!(Solution::k_length_apart(vec![1, 0, 0, 1, 0, 1], 2), false);
    }
}

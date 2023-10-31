// https://leetcode.com/problems/find-the-original-array-of-prefix-xor/
// 2433. Find The Original Array of Prefix Xor
pub struct Solution;
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut pref = pref;
        for i in (1..pref.len()).rev() {
            pref[i] ^= pref[i - 1];
        }
        pref
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_array() {
        assert_eq!(
            Solution::find_array(vec![5, 2, 0, 3, 1]),
            vec![5, 7, 2, 3, 2]
        );
        assert_eq!(Solution::find_array(vec![13]), vec![13]);
    }
}

// https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-ii/
// 2840. Check if Strings Can Be Made Equal With Operations II
pub struct Solution;
impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        let mut even = [0_i32; 26];
        let mut odd = [0_i32; 26];

        for i in 0..b1.len() {
            let idx1 = (b1[i] - b'a') as usize;
            let idx2 = (b2[i] - b'a') as usize;
            if i % 2 == 0 {
                even[idx1] += 1;
                even[idx2] -= 1;
            } else {
                odd[idx1] += 1;
                odd[idx2] -= 1;
            }
        }

        even.iter().all(|&v| v == 0) && odd.iter().all(|&v| v == 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_strings() {
        assert_eq!(
            Solution::check_strings("abcdba".to_string(), "cabdab".to_string()),
            true
        );
        assert_eq!(Solution::check_strings("abe".to_string(), "bea".to_string()), false);
    }
}

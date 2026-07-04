// https://leetcode.com/problems/apply-bitwise-operations-to-make-strings-equal/
// 2546. Apply Bitwise Operations to Make Strings Equal
pub struct Solution;
impl Solution {
    pub fn make_strings_equal(s: String, target: String) -> bool {
        if s == target {
            return true;
        }
        let has_one_in_s = s.contains('1');
        let has_one_in_target = target.contains('1');
        has_one_in_s && has_one_in_target
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_strings_equal() {
        assert_eq!(
            Solution::make_strings_equal("1010".to_string(), "0110".to_string()),
            true
        );
        assert_eq!(Solution::make_strings_equal("11".to_string(), "00".to_string()), false);
    }
}

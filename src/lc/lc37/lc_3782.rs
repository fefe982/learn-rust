// https://leetcode.com/problems/last-remaining-integer-after-alternating-deletion-operations/
// 3782. Last Remaining Integer After Alternating Deletion Operations
pub struct Solution;
impl Solution {
    pub fn last_integer(n: i64) -> i64 {
        ((n - 1) & 0x2aaaaaaaaaaaaaaa) + 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn last_integer() {
        assert_eq!(Solution::last_integer(8), 3);
        assert_eq!(Solution::last_integer(5), 1);
        assert_eq!(Solution::last_integer(1), 1);
    }
}

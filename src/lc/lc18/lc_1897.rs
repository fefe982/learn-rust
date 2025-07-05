// https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/
// 1897. Redistribute Characters to Make All Strings Equal
pub struct Solution;
impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut cnt = [0; 26];
        let n = words.len();
        for w in words {
            for c in w.bytes() {
                cnt[(c - b'a') as usize] += 1;
            }
        }
        cnt.iter().all(|&x| x % n == 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_make_equal() {
        assert_eq!(Solution::make_equal(vec_str!["abc", "aabc", "bc"]), true);
        assert_eq!(Solution::make_equal(vec_str!["ab", "a"]), false);
    }
}

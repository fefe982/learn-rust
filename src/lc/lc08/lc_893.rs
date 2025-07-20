// https://leetcode.com/problems/groups-of-special-equivalent-strings/
// 893. Groups of Special-Equivalent Strings
pub struct Solution;
impl Solution {
    pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
        fn cnt(w: String) -> ([i32; 26], [i32; 26]) {
            let mut cnt1 = [0; 26];
            let mut cnt2 = [0; 26];
            for (i, c) in w.chars().enumerate() {
                if i & 1 == 0 {
                    cnt1[(c as u8 - 'a' as u8) as usize] += 1;
                } else {
                    cnt2[(c as u8 - 'a' as u8) as usize] += 1;
                }
            }
            (cnt1, cnt2)
        }
        let mut set = std::collections::HashSet::new();
        for w in words {
            set.insert(cnt(w));
        }
        set.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn num_special_equiv_groups() {
        assert_eq!(
            Solution::num_special_equiv_groups(vec_str!["abcd", "cdab", "cbad", "xyzz", "zzxy", "zzyx"]),
            3
        );
        assert_eq!(
            Solution::num_special_equiv_groups(vec_str!["abc", "acb", "bac", "bca", "cab", "cba"]),
            3
        );
    }
}

// https://leetcode.com/problems/longest-uncommon-subsequence-ii/
// 522. Longest Uncommon Subsequence II
pub struct Solution;
impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut strs = strs;
        strs.sort_by(|a, b| match b.len().cmp(&a.len()) {
            std::cmp::Ordering::Equal => a.cmp(b),
            ord => ord,
        });
        let mut remove = vec![false; strs.len()];
        for i in 0..strs.len() {
            if remove[i] {
                continue;
            }
            if i + 1 < strs.len() && strs[i] == strs[i + 1] {
                remove[i] = true;
                remove[i + 1] = true;
                let s1 = strs[i].as_bytes();
                for j in i + 2..strs.len() {
                    let s2 = strs[j].as_bytes();
                    let mut i1 = 0;
                    let mut i2 = 0;
                    while i1 < s1.len() && i2 < s2.len() && s2.len() - i2 <= s1.len() - i1 {
                        if s1[i1] == s2[i2] {
                            i1 += 1;
                            i2 += 1;
                        } else {
                            i1 += 1;
                        }
                    }
                    if i2 == s2.len() {
                        remove[j] = true;
                    }
                }
                continue;
            }
            return strs[i].len() as i32;
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_lu_slength() {
        assert_eq!(Solution::find_lu_slength(vec_str!["aba", "cdc", "eae"]), 3);
        assert_eq!(Solution::find_lu_slength(vec_str!["aaa", "aaa", "aa"]), -1);
    }
}

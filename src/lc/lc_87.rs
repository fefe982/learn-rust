// https://leetcode.com/problems/scramble-string/
// 87. Scramble String
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn is_scramble_slice<'a>(s1: &'a [u8], s2: &'a [u8], cache: &mut HashMap<(&'a [u8], &'a [u8]), bool>) -> bool {
        if s1 == s2 {
            return true;
        }
        if s1.len() == 1 {
            return false;
        }
        if let Some(res) = cache.get(&(s1, s2)) {
            return *res;
        }
        let mut s1_head: HashMap<u8, i32> = HashMap::new();
        let mut s2_head: HashMap<u8, i32> = HashMap::new();
        let mut s2_tail: HashMap<u8, i32> = HashMap::new();
        for idx in 0..s1.len() - 1 {
            *s1_head.entry(s1[idx]).or_default() += 1;
            *s2_head.entry(s2[idx]).or_default() += 1;
            *s2_tail.entry(s2[s2.len() - idx - 1]).or_default() += 1;
            if (s1_head == s2_head
                && Self::is_scramble_slice(&s1[0..idx + 1], &s2[0..idx + 1], cache)
                && Self::is_scramble_slice(&s1[idx + 1..], &s2[idx + 1..], cache))
                || (s1_head == s2_tail
                    && Self::is_scramble_slice(&s1[0..idx + 1], &s2[s2.len() - idx - 1..], cache)
                    && Self::is_scramble_slice(&s1[idx + 1..], &s2[0..s2.len() - idx - 1], cache))
            {
                cache.insert((s1, s2), true);
                return true;
            }
        }
        cache.insert((s1, s2), false);
        false
    }
    pub fn is_scramble(s1: String, s2: String) -> bool {
        Self::is_scramble_slice(s1.as_bytes(), s2.as_bytes(), &mut HashMap::new())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_scramble() {
        assert_eq!(
            Solution::is_scramble(String::from("great"), String::from("rgeat")),
            true
        );
        assert_eq!(
            Solution::is_scramble(String::from("abcde"), String::from("caebd")),
            false
        );
        assert_eq!(Solution::is_scramble(String::from("a"), String::from("a")), true);
    }
}

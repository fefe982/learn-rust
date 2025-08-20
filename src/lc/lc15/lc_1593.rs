// https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/
// 1593. Split a String Into the Max Number of Unique Substrings
pub struct Solution;
impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let mut set = std::collections::HashSet::new();
        let s = s.as_bytes();
        let mut stk = vec![(0, 1)];
        set.insert(&s[0..1]);
        let mut max = 0;
        let mut nexts = 1;
        let mut nexte = 2;
        loop {
            while nexte <= s.len() && set.contains(&s[nexts..nexte]) {
                nexte += 1;
            }
            if nexte == s.len() {
                max = max.max(set.len() as i32 + 1);
            }
            if nexte >= s.len() || set.len() + 1 + (s.len() - nexte) <= max as usize {
                if let Some((ps, pe)) = stk.pop() {
                    set.remove(&s[ps..pe]);
                    nexts = ps;
                    nexte = pe + 1;
                } else {
                    break;
                }
            } else {
                stk.push((nexts, nexte));
                set.insert(&s[nexts..nexte]);
                nexts = nexte;
                nexte += 1;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_unique_split() {
        assert_eq!(Solution::max_unique_split("ababccc".to_string()), 5);
        assert_eq!(Solution::max_unique_split("aba".to_string()), 2);
        assert_eq!(Solution::max_unique_split("aa".to_string()), 1);
    }
}

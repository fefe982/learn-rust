// https://leetcode.com/problems/longest-balanced-substring-ii/
// 3714. Longest Balanced Substring II
pub struct Solution;
impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let mut mabc = std::collections::HashMap::new();
        mabc.insert((0, 0), 0);
        let mut mab = std::collections::HashMap::new();
        mab.insert(0, 0);
        let mut mac = std::collections::HashMap::new();
        mac.insert(0, 0);
        let mut mbc = std::collections::HashMap::new();
        mbc.insert(0, 0);
        let mut abcab = 0;
        let mut abcac = 0;
        let mut ab = 0;
        let mut ac = 0;
        let mut bc = 0;
        let mut ans = 0;
        let mut la = 0;
        let mut lb = 0;
        let mut lc = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                'a' => {
                    la += 1;
                    ans = ans.max(la);
                    lb = 0;
                    lc = 0;
                    ab += 1;
                    ac += 1;
                    bc = 0;
                    mbc.clear();
                    abcab += 1;
                    abcac += 1;
                }
                'b' => {
                    la = 0;
                    lb += 1;
                    ans = ans.max(lb);
                    lc = 0;
                    ab -= 1;
                    ac = 0;
                    mac.clear();
                    bc += 1;
                    abcab -= 1;
                }
                'c' => {
                    la = 0;
                    lb = 0;
                    lc += 1;
                    ans = ans.max(lc);
                    ab = 0;
                    mab.clear();
                    ac -= 1;
                    bc -= 1;
                    abcac -= 1;
                }
                _ => {}
            }
            if let Some(j) = mabc.get(&(abcab, abcac)) {
                ans = ans.max(i - j + 1);
            } else {
                mabc.insert((abcab, abcac), i + 1);
            }
            if let Some(j) = mab.get(&ab) {
                ans = ans.max(i - j + 1);
            } else {
                mab.insert(ab, i + 1);
            }
            if let Some(j) = mac.get(&ac) {
                ans = ans.max(i - j + 1);
            } else {
                mac.insert(ac, i + 1);
            }
            if let Some(j) = mbc.get(&bc) {
                ans = ans.max(i - j + 1);
            } else {
                mbc.insert(bc, i + 1);
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_balanced() {
        assert_eq!(Solution::longest_balanced("abcbc".to_string()), 4);
        assert_eq!(Solution::longest_balanced("abbac".to_string()), 4);
        assert_eq!(Solution::longest_balanced("aabcc".to_string()), 3);
        assert_eq!(Solution::longest_balanced("aba".to_string()), 2);
    }
}

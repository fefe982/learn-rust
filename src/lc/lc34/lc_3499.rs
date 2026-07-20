// https://leetcode.com/problems/maximize-active-section-with-trade-i/
// 3499. Maximize Active Sections With Trade-Offs
pub struct Solution;
impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let mut c1 = 0;
        let mut l0 = 0;
        let mut c0 = 0;
        let mut max = 0;
        for c in s.chars() {
            if c == '1' {
                c1 += 1;
                if c0 > 0 {
                    if l0 > 0 {
                        max = max.max(l0 + c0);
                    }
                    l0 = c0;
                    c0 = 0;
                }
            } else {
                c0 += 1;
            }
        }
        if l0 > 0 && c0 > 0 {
            max = max.max(l0 + c0);
        }
        c1 + max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_active_sections_after_trade() {
        assert_eq!(Solution::max_active_sections_after_trade("01".to_string()), 1);
        assert_eq!(Solution::max_active_sections_after_trade("0100".to_string()), 4);
        assert_eq!(Solution::max_active_sections_after_trade("1000100".to_string()), 7);
        assert_eq!(Solution::max_active_sections_after_trade("01010".to_string()), 4);
    }
}

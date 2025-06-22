// https://leetcode.com/problems/integer-to-roman/
// 12. Integer to Roman
pub struct Solution;
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let r3 = vec!["", "M", "MM", "MMM"];
        let r2 = vec!["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        let r1 = vec!["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        let r0 = vec!["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
        let mut s = String::new();
        s.push_str(r3[(num / 1000) as usize]);
        s.push_str(r2[((num % 1000) / 100) as usize]);
        s.push_str(r1[((num % 100) / 10) as usize]);
        s.push_str(r0[(num % 10) as usize]);
        s
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn int_to_roman() {
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}

// https://leetcode.com/problems/english-int-lcci/
// 面试题 16.08. English Int LCCI
pub struct Solution;
impl Solution {
    pub fn number_to_words(num: i32) -> String {
        let digit = [
            "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
        ];
        let ten = [
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];
        let ten2 = [
            "Z", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];
        let hundred = "Hundred";
        let sep = ["Thousand", "Million", "Billion"];
        let make_thousand = |num: i32| -> Vec<&str> {
            let mut ret = vec![];
            let h = num / 100;
            if h > 0 {
                ret.push(digit[h as usize]);
                ret.push(hundred);
            }
            let t = (num % 100) / 10;
            let d = num % 10;
            if t == 1 {
                ret.push(ten[d as usize]);
            } else {
                if t > 1 {
                    ret.push(ten2[t as usize]);
                }
                if d > 0 {
                    ret.push(digit[d as usize]);
                }
            }
            ret
        };
        let mut ret = vec![];
        let b = num / 1_000_000_000;
        if b > 0 {
            ret = make_thousand(b);
            ret.push(sep[2]);
        }
        let m = (num % 1_000_000_000) / 1_000_000;
        if m > 0 {
            ret.extend(make_thousand(m));
            ret.push(sep[1]);
        }
        let t = (num % 1_000_000) / 1_000;
        if t > 0 {
            ret.extend(make_thousand(t));
            ret.push(sep[0]);
        }
        let o = num % 1_000;
        if o > 0 {
            ret.extend(make_thousand(o));
        }
        if ret.is_empty() {
            "Zero".to_string()
        } else {
            ret.join(" ").to_string()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_to_words() {
        assert_eq!(Solution::number_to_words(123), "One Hundred Twenty Three".to_string());
        assert_eq!(
            Solution::number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five".to_string()
        );
        assert_eq!(
            Solution::number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_string()
        );
        assert_eq!(
            Solution::number_to_words(1234567891),
            "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
                .to_string()
        );
    }
}

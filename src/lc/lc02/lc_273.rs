// https://leetcode.com/problems/integer-to-english-words/
// 273. Integer to English Words
pub struct Solution;
impl Solution {
    fn read_low(mut num: i32, res: &mut Vec<u8>) {
        let digit_arr = [
            "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
        ]
        .map(|x| x.as_bytes());
        let ty_arr = [
            "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ]
        .map(|x| x.as_bytes());
        let teen_arr = [
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
        ]
        .map(|x| x.as_bytes());
        if num >= 100 {
            res.extend_from_slice(digit_arr[(num / 100) as usize]);
            res.extend_from_slice(" Hundred ".as_bytes());
            num %= 100;
        }
        if num >= 10 && num <= 19 {
            res.extend_from_slice(teen_arr[(num - 10) as usize]);
            res.push(b' ');
        } else {
            if num >= 10 {
                res.extend_from_slice(ty_arr[(num / 10) as usize]);
                res.push(b' ');
                num %= 10;
            }
            if num > 0 {
                res.extend_from_slice(digit_arr[num as usize]);
                res.push(b' ');
            }
        }
    }
    pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            return String::from("Zero");
        }
        let mut id = 0;
        let mut d = 1;
        while num / d >= 1000 {
            d *= 1000;
            id += 1;
        }
        let unit_arr = ["", "Thousand ", "Million ", "Billion "].map(|x| x.as_bytes());
        let mut res = Vec::new();
        while num > 0 {
            let high = num / d;
            if high > 0 {
                Self::read_low(high, &mut res);
                res.extend_from_slice(unit_arr[id as usize]);
            }
            num %= d;
            d /= 1000;
            id -= 1;
        }
        res.pop();
        String::from_utf8(res).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_to_words() {
        assert_eq!(
            Solution::number_to_words(123),
            String::from("One Hundred Twenty Three")
        );
        assert_eq!(
            Solution::number_to_words(12345),
            String::from("Twelve Thousand Three Hundred Forty Five")
        );
        assert_eq!(
            Solution::number_to_words(1234567),
            String::from("One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven")
        );
    }
}

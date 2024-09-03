// https://leetcode.com/problems/sum-of-digits-of-string-after-convert/
// 1945. Sum of Digits of String After Convert
pub struct Solution;
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut sum = s
            .as_bytes()
            .into_iter()
            .map(|c| (*c - b'a') as i32 % 9 + if *c == b's' { 10 } else { 1 })
            .sum::<i32>();
        for _ in 1..k {
            if sum < 10 {
                return sum;
            }
            let mut nsum = 0;
            while sum > 0 {
                nsum += sum % 10;
                sum /= 10;
            }
            sum = nsum;
        }
        sum
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn get_lucky() {
        assert_eq!(Solution::get_lucky("hvmhoasabaymnmsd".to_string(), 1), 79);
        assert_eq!(Solution::get_lucky("iiii".to_string(), 1), 36);
        assert_eq!(Solution::get_lucky("leetcode".to_string(), 2), 6);
        assert_eq!(Solution::get_lucky("zbax".to_string(), 2), 8);
    }
}

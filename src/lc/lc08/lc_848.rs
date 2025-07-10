// https://leetcode.com/problems/shifting-letters/
// 848. Shifting Letters
pub struct Solution;
impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut shifts = shifts;
        let n = shifts.len();
        shifts[n - 1] %= 26;
        for i in (0..n - 1).rev() {
            shifts[i] = (shifts[i] + shifts[i + 1]) % 26;
        }
        s.chars()
            .zip(shifts.into_iter())
            .map(|(c, s)| ((c as u8 - b'a' + s as u8) % 26 + b'a') as char)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shifting_letters() {
        assert_eq!(Solution::shifting_letters("abc".to_string(), vec![3, 5, 9]), "rpl");
        assert_eq!(Solution::shifting_letters("aaa".to_string(), vec![1, 2, 3]), "gfd");
    }
}

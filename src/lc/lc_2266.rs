// https://leetcode.com/problems/count-number-of-texts/
// 2266. Count Number of Texts
pub struct Solution;
impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        let mut last = [0; 4];
        last[0] = 1;
        let mut lc = '0';
        for c in pressed_keys.chars() {
            if c == lc {
                let n = if c == '7' || c == '9' {
                    last[0] + last[1] + last[2] + last[3]
                } else {
                    last[0] + last[1] + last[2]
                } % 1_000_000_007i64;
                last[3] = last[2];
                last[2] = last[1];
                last[1] = last[0];
                last[0] = n;
            } else {
                last[1] = last[0];
                last[2] = 0;
                last[3] = 0;
            }
            lc = c;
        }
        last[0] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_texts() {
        assert_eq!(Solution::count_texts("22233".to_string()), 8);
        assert_eq!(
            Solution::count_texts("222222222222222222222222222222222222".to_string()),
            82876089
        );
    }
}

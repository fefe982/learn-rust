// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/
// 1461. Check If a String Contains All Binary Codes of Size K
pub struct Solution;
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut v = vec![0; 1 << k];
        let mask = (1 << k) - 1;
        let mut val = 0;
        let mut cnt = 0;
        let s = s.chars().collect::<Vec<char>>();
        let k = k as usize;
        for i in 0..s.len() {
            let bit = if s[i] == '1' { 1 } else { 0 };
            val = ((val << 1) | bit) & mask;
            if i >= k - 1 && v[val] == 0 {
                cnt += 1;
                v[val] = 1;
            }
        }
        cnt == 1 << k
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_all_codes() {
        assert_eq!(Solution::has_all_codes("00110110".to_string(), 2), true);
        assert_eq!(Solution::has_all_codes("0110".to_string(), 1), true);
        assert_eq!(Solution::has_all_codes("0110".to_string(), 2), false);
    }
}

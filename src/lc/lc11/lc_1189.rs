// https://leetcode.com/problems/maximum-number-of-balloons/
// 1189. Maximum Number of Balloons
pub struct Solution;
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut cnt = vec![0; 26];
        for c in text.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        cnt[0].min(cnt[1]).min(cnt[11] / 2).min(cnt[13]).min(cnt[14] / 2)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_number_of_balloons() {
        assert_eq!(Solution::max_number_of_balloons("nlaebolko".to_string()), 1);
        assert_eq!(Solution::max_number_of_balloons("loonbalxballpoon".to_string()), 2);
    }
}

// https://leetcode.com/problems/minimum-distance-to-type-a-word-using-two-fingers/
// 1320. Minimum Distance to Type a Word Using Two Fingers
pub struct Solution;
impl Solution {
    fn d(a: i32, b: i32) -> i32 {
        (a / 6 - b / 6).abs() + (a % 6 - b % 6).abs()
    }
    pub fn minimum_distance(word: String) -> i32 {
        let word = word.bytes().map(|c| (c - b'A') as i32).collect::<Vec<_>>();
        let mut dp = vec![0; 26];
        let mut sum = Self::d(word[0], word[1]);
        dp[word[0] as usize] = sum;
        for i in 1..word.len() - 1 {
            let b = word[i];
            let c = word[i + 1];
            let dbc = Self::d(b, c);
            sum += dbc;
            let mut save = i32::MIN;
            for a in 0..26 {
                save = save.max(dp[a as usize] + dbc - Self::d(a, c));
            }
            dp[b as usize] = save;
        }
        sum - dp.into_iter().max().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_distance() {
        assert_eq!(Solution::minimum_distance(String::from("CAKE")), 3);
        assert_eq!(Solution::minimum_distance(String::from("HAPPY")), 6);
    }
}

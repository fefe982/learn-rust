// https://leetcode.com/problems/vowels-game-in-a-string/
// 3227. Vowels Game in a String
pub struct Solution;
impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.chars()
            .any(|c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn does_alice_win() {
        assert_eq!(Solution::does_alice_win("leetcode".to_string()), true);
        assert_eq!(Solution::does_alice_win("bbcd".to_string()), false);
    }
}

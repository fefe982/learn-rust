// https://leetcode.com/problems/remove-colored-pieces-if-both-neighbors-are-the-same-color/
// 2038. Remove Colored Pieces if Both Neighbors are the Same Color
pub struct Solution;
impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut ca = 0;
        let mut cb = 0;
        let mut ra = 0;
        let mut rb = 0;
        for c in colors.chars().chain(".".chars()) {
            if c != 'A' {
                if ca > 2 {
                    ra += ca - 2;
                }
                ca = 0;
            }
            if c != 'B' {
                if cb > 2 {
                    rb += cb - 2;
                }
                cb = 0;
            }
            if c == 'A' {
                ca += 1;
            } else if c == 'B' {
                cb += 1;
            }
        }
        ra > rb
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_winner_of_game() {
        assert_eq!(Solution::winner_of_game("AAABABB".to_string()), true);
        assert_eq!(Solution::winner_of_game("AA".to_string()), false);
        assert_eq!(Solution::winner_of_game("ABBBBBBBAAA".to_string()), false);
    }
}

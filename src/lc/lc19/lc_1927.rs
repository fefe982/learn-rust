// https://leetcode.com/problems/sum-game/
// 1927. Sum Game
pub struct Solution;
impl Solution {
    pub fn sum_game(num: String) -> bool {
        let num = num.as_bytes();
        let mut sum = 0;
        let n = num.len();
        for i in 0..n / 2 {
            if num[i] == b'?' {
                sum += 9;
            } else {
                sum += (num[i] - b'0') as i32 * 2;
            }
        }
        for i in n / 2..n {
            if num[i] == b'?' {
                sum -= 9;
            } else {
                sum -= (num[i] - b'0') as i32 * 2;
            }
        }
        sum != 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_game() {
        assert_eq!(Solution::sum_game("5023".to_string()), false);
        assert_eq!(Solution::sum_game("25??".to_string()), true);
        assert_eq!(Solution::sum_game("?3295???".to_string()), false);
    }
}

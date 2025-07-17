// https://leetcode.com/problems/number-of-laser-beams-in-a-bank/
// 2125. Number of Laser Beams in a Bank
pub struct Solution;
impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.into_iter()
            .map(|s| s.chars().filter(|&c| c == '1').count() as i32)
            .fold((0, 0), |(sum, last), cur| {
                if cur != 0 {
                    if last != 0 {
                        (sum + cur * last, cur)
                    } else {
                        (sum, cur)
                    }
                } else {
                    (sum, last)
                }
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_number_of_beams() {
        assert_eq!(
            Solution::number_of_beams(vec_str!["011001", "000000", "010100", "001000"]),
            8
        );
        assert_eq!(Solution::number_of_beams(vec_str!["000", "111", "000"]), 0);
    }
}

// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
// 17. Letter Combinations of a Phone Number
pub struct Solution;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let dc = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        let digits = digits
            .bytes()
            .map(|c| (c - b'2') as usize)
            .collect::<Vec<usize>>();
        let mut ord = vec![0; digits.len()];
        let mut res = vec![];
        loop {
            res.push(
                ord.iter()
                    .enumerate()
                    .map(|(d, &i)| dc[digits[d]][i])
                    .collect(),
            );
            for i in (0..ord.len()).rev() {
                ord[i] += 1;
                if ord[i] < dc[digits[i]].len() {
                    for j in i + 1..ord.len() {
                        ord[j] = 0;
                    }
                    break;
                }
                if i == 0 {
                    return res;
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn is_valid() {
        assert_eq!(
            Solution::letter_combinations("3".to_string()),
            vec_str!["d", "e", "f"]
        );
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec_str!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec_str!["a", "b", "c"]
        );
    }
}

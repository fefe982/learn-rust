// https://leetcode.com/problems/special-binary-string/
// 761. Special Binary String
pub struct Solution;
impl Solution {
    fn make_vec(s: Vec<char>) -> Vec<char> {
        let mut res = vec![];
        let mut count = 0;
        let mut last = 0;
        for (i, &c) in s.iter().enumerate() {
            if c == '1' {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                res.push(
                    "1".chars()
                        .chain(Solution::make_vec(s[last + 1..i].iter().cloned().collect::<Vec<_>>()).into_iter())
                        .chain("0".chars())
                        .collect::<Vec<_>>(),
                );
                last = i + 1;
            }
        }
        res.sort_by(|s1, s2| s2.cmp(s1));
        res.into_iter().flatten().collect::<Vec<_>>()
    }
    pub fn make_largest_special(s: String) -> String {
        Self::make_vec(s.chars().collect::<Vec<_>>())
            .into_iter()
            .collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_make_largest_special() {
        assert_eq!(
            Solution::make_largest_special("11011000".to_string()),
            "11100100".to_string()
        );
        assert_eq!(Solution::make_largest_special("10".to_string()), "10".to_string());
    }
}

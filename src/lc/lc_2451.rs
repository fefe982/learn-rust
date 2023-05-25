// https://leetcode.com/problems/odd-string-difference/
// 2451. Odd String Difference
pub struct Solution;
impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        for i in 1..words[0].as_bytes().len() {
            if words[0].as_bytes()[i] - words[0].as_bytes()[0]
                == words[1].as_bytes()[i] - words[1].as_bytes()[0]
            {
                for j in 2..words.len() {
                    if words[j].as_bytes()[i] - words[j].as_bytes()[0]
                        != words[0].as_bytes()[i] - words[0].as_bytes()[0]
                    {
                        return words[j].clone();
                    }
                }
            } else if words[2].as_bytes()[i] - words[2].as_bytes()[0]
                == words[0].as_bytes()[i] - words[0].as_bytes()[0]
            {
                return words[1].clone();
            } else {
                return words[0].clone();
            }
        }
        String::from("")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn odd_string() {
        assert_eq!(
            Solution::odd_string(vec_str!["adc", "wzy", "abc"]),
            String::from("abc")
        );
        assert_eq!(
            Solution::odd_string(vec_str!["aaa", "bob", "ccc", "ddd"]),
            String::from("bob")
        );
    }
}

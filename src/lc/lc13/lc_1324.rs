// https://leetcode.com/problems/print-words-vertically/
// 1324. Print Words Vertically
pub struct Solution;
impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        for (i, word) in s.split(' ').enumerate() {
            for (j, c) in word.chars().enumerate() {
                if ans.len() <= j {
                    ans.push(std::iter::repeat(' ').take(i).collect());
                } else if ans[j].len() < i {
                    let l = i - ans[j].len();
                    ans[j].extend(std::iter::repeat(' ').take(l));
                }
                ans[j].push(c);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn print_vertically() {
        assert_eq!(
            Solution::print_vertically("HOW ARE YOU".to_string()),
            vec_str!["HAY", "ORO", "WEU"]
        );
        assert_eq!(
            Solution::print_vertically("TO BE OR NOT TO BE".to_string()),
            vec_str!["TBONTB", "OEROOE", "   T"]
        );
        assert_eq!(
            Solution::print_vertically("CONTEST IS COMING".to_string()),
            vec_str!["CIC", "OSO", "N M", "T I", "E N", "S G", "T"]
        );
    }
}

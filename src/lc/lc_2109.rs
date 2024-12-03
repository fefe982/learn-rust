// https://leetcode.com/problems/adding-spaces-to-a-string/
// 2109. Adding Spaces to a String
pub struct Solution;
impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut res = String::with_capacity(s.len() + spaces.len());
        let mut i = 0;
        let mut is = 0;
        for c in s.chars() {
            if is < spaces.len() && i == spaces[is] {
                res.push(' ');
                is += 1;
            }
            res.push(c);
            i += 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_spaces() {
        assert_eq!(
            Solution::add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15]),
            "Leetcode Helps Me Learn"
        );
        assert_eq!(
            Solution::add_spaces("icodeinpython".to_string(), vec![1, 5, 7, 9]),
            "i code in py thon"
        );
        assert_eq!(
            Solution::add_spaces("spacing".to_string(), vec![0, 1, 2, 3, 4, 5, 6]),
            " s p a c i n g"
        );
    }
}

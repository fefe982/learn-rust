// https://leetcode.com/problems/goal-parser-interpretation/
// 1678. Goal Parser Interpretation
pub struct Solution;
impl Solution {
    pub fn interpret(command: String) -> String {
        let mut res = "".to_string();
        let mut lc = '.';
        for c in command.chars() {
            if c == 'G' {
                res.push(c);
            } else if c == ')' {
                if lc == '(' {
                    res.push('o');
                } else {
                    res.push('a');
                    res.push('l');
                }
            }
            lc = c;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interpret() {
        assert_eq!(Solution::interpret(String::from("G()(al)")), String::from("Goal"));
        assert_eq!(
            Solution::interpret(String::from("G()()()()(al)")),
            String::from("Gooooal")
        );
        assert_eq!(
            Solution::interpret(String::from("(al)G(al)()()G")),
            String::from("alGalooG")
        );
    }
}

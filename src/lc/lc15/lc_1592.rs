// https://leetcode.com/problems/rearrange-spaces-between-words/
// 1592. Rearrange Spaces Between Words
pub struct Solution;
impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let mut result = Vec::new();
        let mut last_end = 0;
        let mut in_whitespace = false;
        let mut nwhitespace = 0;

        for (i, c) in text.chars().enumerate() {
            let is_whitespace = c.is_ascii_whitespace();
            if is_whitespace {
                nwhitespace += 1;
            }
            if i == 0 {
                in_whitespace = is_whitespace;
                continue;
            }
            if is_whitespace && !in_whitespace {
                result.push(&text[last_end..i]);
            }
            if is_whitespace != in_whitespace {
                last_end = i;
                in_whitespace = is_whitespace;
            }
        }
        if last_end < text.len() && !in_whitespace {
            result.push(&text[last_end..]);
        }
        let nsp;
        if result.len() > 1 {
            nsp = nwhitespace / (result.len() - 1);
        } else {
            nsp = 0;
        }
        let mut ans = String::new();
        for (i, s) in result.iter().enumerate() {
            ans.push_str(s);
            if i < result.len() - 1 {
                ans.push_str(&" ".repeat(nsp));
            } else {
                ans.push_str(&" ".repeat(nwhitespace - nsp * (result.len() - 1)));
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reorder_spaces() {
        assert_eq!(
            Solution::reorder_spaces("  this   is  a sentence ".to_string()),
            "this   is   a   sentence".to_string()
        );
        assert_eq!(
            Solution::reorder_spaces(" practice   makes   perfect".to_string()),
            "practice   makes   perfect ".to_string()
        );
    }
}

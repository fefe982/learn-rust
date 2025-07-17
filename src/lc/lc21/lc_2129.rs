// https://leetcode.com/problems/capitalize-the-title/
// 2129. Capitalize the Title
pub struct Solution;
impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let mut chs: Vec<char> = vec![];
        let mut last = 0;
        for (i, c) in title.chars().enumerate() {
            if c == ' ' {
                if i > last + 2 {
                    chs[last] = chs[last].to_ascii_uppercase()
                }
                last = i + 1;
                chs.push(' ');
            } else {
                chs.push(c.to_ascii_lowercase());
            }
        }
        if last + 2 < title.len() {
            chs[last] = chs[last].to_ascii_uppercase();
        }
        chs.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::capitalize_title("a".to_string()), "a");
        assert_eq!(Solution::capitalize_title("ZW Cl pyR uoC".to_string()), "zw cl Pyr Uoc");
        assert_eq!(
            Solution::capitalize_title("capiTalIze tHe titLe".to_string()),
            "Capitalize The Title"
        );
        assert_eq!(
            Solution::capitalize_title("First leTTeR of EACH Word".to_string()),
            "First Letter of Each Word"
        );
        assert_eq!(
            Solution::capitalize_title("i lOve leetcode".to_string()),
            "i Love Leetcode"
        );
    }
}

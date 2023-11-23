// https://leetcode.com/problems/html-entity-parser/
// 1410. HTML Entity Parser
pub struct Solution;
impl Solution {
    pub fn entity_parser(text: String) -> String {
        text.replace("&quot;", "\"")
            .replace("&apos;", "'")
            .replace("&gt;", ">")
            .replace("&lt;", "<")
            .replace("&frasl;", "/")
            .replace("&amp;", "&")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_entity_parser() {
        assert_eq!(
            Solution::entity_parser("&amp; is an HTML entity but &ambassador; is not.".to_string()),
            "& is an HTML entity but &ambassador; is not.".to_string()
        );
        assert_eq!(
            Solution::entity_parser("and I quote: &quot;...&quot;".to_string()),
            "and I quote: \"...\"".to_string()
        );
    }
}

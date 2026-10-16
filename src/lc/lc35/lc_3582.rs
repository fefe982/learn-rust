// https://leetcode.com/problems/generate-tag-for-video-caption/
// 3582. Generate Tag for Video Caption
pub struct Solution;
impl Solution {
    pub fn generate_tag(caption: String) -> String {
        let mut tag = "#".to_string();
        let mut start = false;
        let mut first = true;
        for c in caption.chars() {
            if c.is_alphabetic() {
                if start && !first {
                    tag.push(c.to_ascii_uppercase());
                } else {
                    tag.push(c.to_ascii_lowercase());
                }
                first = false;
                start = false;
            } else if c.is_whitespace() {
                start = true;
            }
            if tag.len() >= 100 {
                break;
            }
        }
        tag
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_tag() {
        assert_eq!(
            Solution::generate_tag(" fPysaRtLQLiMKVvRhMkkDLNedQKffPnCjbITBTOVhoVjiKbfSawvpisDaNzXJctQkn".to_string()),
            "#fpysartlqlimkvvrhmkkdlnedqkffpncjbitbtovhovjikbfsawvpisdanzxjctqkn".to_string()
        );
        assert_eq!(
            Solution::generate_tag("Leetcode daily streak achieved".to_string()),
            "#leetcodeDailyStreakAchieved".to_string()
        );
        assert_eq!(
            Solution::generate_tag("can I Go There".to_string()),
            "#canIGoThere".to_string()
        );
        assert_eq!(
            Solution::generate_tag(
                "hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh"
                    .to_string()
            ),
            "#hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh"
                .to_string()
        );
    }
}

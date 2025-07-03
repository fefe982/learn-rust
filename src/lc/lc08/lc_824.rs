// https://leetcode.com/problems/goat-latin/
// 824. Goat Latin
pub struct Solution;
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        sentence
            .split_ascii_whitespace()
            .enumerate()
            .map(|(i, s)| {
                let mut r = String::with_capacity(s.len() + i + 4);
                if i > 0 {
                    r.push(' ');
                }
                let c = s.chars().next().unwrap();
                if c == 'a'
                    || c == 'e'
                    || c == 'i'
                    || c == 'o'
                    || c == 'u'
                    || c == 'A'
                    || c == 'E'
                    || c == 'I'
                    || c == 'O'
                    || c == 'U'
                {
                    r.push_str(s)
                } else {
                    r.push_str(&s[1..]);
                    r.push(c);
                }
                r.push('m');
                r.push_str(&("a".repeat(i + 2)));
                r
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn to_goat_latin() {
        assert_eq!(
            Solution::to_goat_latin("I speak Goat Latin".to_string()),
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
        );
        assert_eq!(Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string()), "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa");
    }
}

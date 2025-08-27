// https://leetcode.com/problems/last-substring-in-lexicographical-order/
// 1163. Last Substring in Lexicographical Order
pub struct Solution;
impl Solution {
    pub fn last_substring(s: String) -> String {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = 1;
        let mut k = 0;
        while j + k < s.len() {
            match s[i + k].cmp(&s[j + k]) {
                std::cmp::Ordering::Greater => {
                    j += k.max(1);
                    k = 0;
                }
                std::cmp::Ordering::Equal => k += 1,
                std::cmp::Ordering::Less => {
                    i = j.max(i + k + 1);
                    k = 0;
                    j = i + 1;
                }
            }
        }
        String::from_utf8(s[i..].to_owned()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn last_substring() {
        assert_eq!(
            Solution::last_substring(String::from("abcdexyzfzz")),
            String::from("zz")
        );
        assert_eq!(Solution::last_substring(String::from("abab")), String::from("bab"));
        assert_eq!(
            Solution::last_substring(String::from("leetcode")),
            String::from("tcode")
        );
    }
}

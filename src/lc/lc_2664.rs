// https://leetcode.cn/problems/lexicographically-smallest-beautiful-string/
// 2663. Lexicographically Smallest Beautiful String
pub struct Solution;
impl Solution {
    pub fn smallest_beautiful_string(s: String, k: i32) -> String {
        let mut s = s.as_bytes().to_vec();
        let mut i = s.len();
        let kc = b'a' + k as u8;
        'i: while i > 0 {
            while s[i - 1] + 1 < kc {
                s[i - 1] += 1;
                if (i > 1 && s[i - 1] == s[i - 2]) || (i > 2 && s[i - 1] == s[i - 3]) {
                    continue;
                }
                break 'i;
            }
            i -= 1;
        }
        if i == 0 {
            "".to_string()
        } else {
            while i < s.len() {
                for c in b'a'.. {
                    if (i > 0 && c == s[i - 1]) || (i > 1 && c == s[i - 2]) {
                        continue;
                    }
                    s[i] = c;
                    break;
                }
                i += 1;
            }
            String::from_utf8(s).unwrap()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smallest_beautiful_string() {
        assert_eq!(Solution::smallest_beautiful_string("abcz".to_string(), 26), "abda");
        assert_eq!(Solution::smallest_beautiful_string("dc".to_string(), 4), "");
    }
}

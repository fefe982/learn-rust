// https://leetcode.com/problems/last-substring-in-lexicographical-order/
// 1163. Last Substring in Lexicographical Order
pub struct Solution;
impl Solution {
    pub fn last_substring(s: String) -> String {
        let s = s.as_bytes();
        let mut m = std::collections::BTreeMap::new();
        let mut max_c = 0;
        for (idx, &c) in s.iter().enumerate() {
            if c > max_c {
                m.clear();
                max_c = c;
            }
            if c == max_c {
                m.insert(idx, idx + 1);
            }
        }
        while m.len() > 1 {
            max_c = 0;
            let mut max_len = 0;
            let mut mm = std::collections::BTreeMap::new();
            let mut last_value = 0;
            for (&key, &value) in m.iter() {
                if value <= last_value {
                    continue;
                }
                let mut value = value;
                while value < s.len() {
                    if let Some(&v) = m.get(&value) {
                        value = v;
                    } else {
                        break;
                    }
                }
                if value == s.len() {
                    value -= 1;
                }
                let len = value + 1 - key;
                let c = s[value];
                if len > max_len || (len == max_len && c > max_c) {
                    mm.clear();
                    max_len = len;
                    max_c = c;
                }
                if len == max_len && c == max_c {
                    mm.insert(key, value + 1);
                }
                last_value = value;
            }
            m = mm;
        }
        for &key in m.keys() {
            return std::str::from_utf8(&s[key..]).unwrap().to_string();
        }
        String::from("")
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
        assert_eq!(
            Solution::last_substring(String::from("abab")),
            String::from("bab")
        );
        assert_eq!(
            Solution::last_substring(String::from("leetcode")),
            String::from("tcode")
        );
    }
}

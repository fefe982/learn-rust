// https://leetcode.com/problems/check-if-an-original-string-exists-given-two-encoded-strings/
// 2060. Check if an Original String Exists Given Two Encoded Strings
pub struct Solution;
impl Solution {
    fn get_digit(s: &[u8]) -> Vec<(i32, &[u8])> {
        if s.len() == 0 {
            return vec![];
        }
        let mut res = vec![];
        let mut i = 0;
        let mut num = 0;
        while i < s.len() && s[i] >= b'0' && s[i] <= b'9' {
            num = num * 10 + (s[i] - b'0') as i32;
            res.push((num, &s[i + 1..]));
            i += 1;
        }
        res
    }
    fn check_digit(
        s: &[u8],
        t: &[u8],
        diff: i32,
        rev: bool,
        cache: &mut Vec<Vec<std::collections::HashMap<i32, bool>>>,
    ) -> Option<bool> {
        let rs = Self::get_digit(s);
        if rs.len() > 0 {
            for (num, rss) in rs {
                if (rev == false && Self::check(rss, t, cache, diff + num))
                    || (rev == true && Self::check(t, rss, cache, -diff - num))
                {
                    return Some(true);
                }
            }
            return Some(false);
        }
        None
    }

    fn check_inner(s: &[u8], t: &[u8], cache: &mut Vec<Vec<std::collections::HashMap<i32, bool>>>, diff: i32) -> bool {
        if diff == 0 {
            if s.len() == 0 || t.len() == 0 {
                return false;
            } else {
                if !s[0].is_ascii_digit() && !t[0].is_ascii_digit() {
                    if s[0] == t[0] {
                        return Self::check(&s[1..], &t[1..], cache, 0);
                    } else {
                        return false;
                    }
                }
                if let Some(value) = Self::check_digit(s, t, 0, false, cache) {
                    return value;
                }
                if let Some(value) = Self::check_digit(t, s, 0, true, cache) {
                    return value;
                }
                return false;
            }
        } else if diff > 0 {
            if t.len() == 0 {
                return false;
            }
            if let Some(value) = Self::check_digit(t, s, -diff, true, cache) {
                return value;
            } else {
                return Self::check(s, &t[1..], cache, diff - 1);
            }
        } else if diff < 0 {
            if s.len() == 0 {
                return false;
            }
            if let Some(value) = Self::check_digit(s, t, diff, false, cache) {
                return value;
            } else {
                return Self::check(&s[1..], t, cache, diff + 1);
            }
        }
        false
    }
    fn check(s: &[u8], t: &[u8], cache: &mut Vec<Vec<std::collections::HashMap<i32, bool>>>, diff: i32) -> bool {
        if s.len() == 0 && t.len() == 0 {
            return diff == 0;
        }
        if !cache[s.len()][t.len()].contains_key(&diff) {
            let res = Self::check_inner(s, t, cache, diff);
            cache[s.len()][t.len()].insert(diff, res);
        }
        *cache[s.len()][t.len()].get(&diff).unwrap()
    }
    pub fn possibly_equals(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        Self::check(
            s1,
            s2,
            &mut vec![vec![std::collections::HashMap::new(); s2.len() + 1]; s1.len() + 1],
            0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    fn wrapper(s1: String, s2: String) -> bool {
        let now = Instant::now();
        let b = Solution::possibly_equals(s1, s2);
        println!("{} ms", now.elapsed().as_millis());
        b
    }
    #[test]
    fn test_possibly_equals() {
        assert_eq!(
            wrapper(
                "g69q533q68q277g4g554g5q8g4q958q298".to_string(),
                "9q87q5g82g444q8g83g81q282q58g473q97".to_string()
            ),
            false
        );
        assert_eq!(Solution::possibly_equals("ab".to_string(), "a2".to_string()), false);
        assert_eq!(
            Solution::possibly_equals("internationalization".to_string(), "i18n".to_string()),
            true
        );
        assert_eq!(Solution::possibly_equals("l123e".to_string(), "44".to_string()), true);
        assert_eq!(Solution::possibly_equals("a5b".to_string(), "c5b".to_string()), false);
    }
}

// https://leetcode.com/problems/minimum-number-of-frogs-croaking/
// 1419. Minimum Number of Frogs Croaking
pub struct Solution;
impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut v = Vec::new();
        let mut cnt = 0;
        let m = std::collections::HashMap::from([(b'r', b'c'), (b'o', b'r'), (b'a', b'o')]);
        for &c in croak_of_frogs.as_bytes() {
            if c == b'c' {
                v.push(c);
                if cnt < v.len() {
                    cnt = v.len();
                }
            } else if c == b'k' {
                let mut idx = v.len();
                while idx > 0 {
                    if v[idx - 1] == b'a' {
                        v.remove(idx - 1);
                        break;
                    }
                    idx -= 1;
                }
                if idx == 0 {
                    return -1;
                }
            } else if let Some(&pre) = m.get(&c) {
                let mut idx = v.len();
                while idx > 0 {
                    if v[idx - 1] == pre {
                        v[idx - 1] = c;
                        break;
                    }
                    idx -= 1;
                }
                if idx == 0 {
                    return -1;
                }
            } else {
                return -1;
            }
        }
        if v.is_empty() {
            cnt as i32
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_number_of_frogs() {
        assert_eq!(Solution::min_number_of_frogs(String::from("croakcroa")), -1);
        assert_eq!(Solution::min_number_of_frogs(String::from("croakcroak")), 1);
        assert_eq!(Solution::min_number_of_frogs(String::from("crcoakroak")), 2);
        assert_eq!(
            Solution::min_number_of_frogs(String::from("croakcrook")),
            -1
        );
    }
}

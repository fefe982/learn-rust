// https://leetcode.com/problems/process-string-with-special-operations-i/
// 3612. Process String With Special Operations I
pub struct Solution;
impl Solution {
    pub fn process_str(s: String) -> String {
        let mut res = std::collections::VecDeque::new();
        let mut fwd = true;
        for c in s.chars() {
            match c {
                '*' => {
                    if res.is_empty() {
                        continue;
                    }
                    if fwd {
                        res.pop_back();
                    } else {
                        res.pop_front();
                    }
                }
                '#' => {
                    let n = res.len();
                    for i in 0..n {
                        let c = res[i];
                        res.push_back(c);
                    }
                }
                '%' => {
                    fwd = !fwd;
                }
                _ => {
                    if fwd {
                        res.push_back(c);
                    } else {
                        res.push_front(c);
                    }
                }
            }
        }
        if fwd {
            res.into_iter().collect()
        } else {
            res.into_iter().rev().collect()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn process_str() {
        assert_eq!(Solution::process_str("a#b%*".to_string()), "ba");
        assert_eq!(Solution::process_str("z*#".to_string()), "");
    }
}

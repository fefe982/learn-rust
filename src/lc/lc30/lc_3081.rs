// https://leetcode.com/problems/replace-question-marks-in-string-to-minimize-its-value/
// 3081. Replace Question Marks in String to Minimize Its Value
pub struct Solution;
impl Solution {
    pub fn minimize_string_value(s: String) -> String {
        let ss = s.as_bytes();
        let mut cnt = vec![0; 26];
        let mut cq = 0;
        for &c in ss {
            if c == b'?' {
                cq += 1;
            } else {
                cnt[(c - b'a') as usize] += 1;
            }
        }
        if cq == 0 {
            return s;
        }
        let mut freq = cnt.iter().cloned().enumerate().collect::<Vec<_>>();
        freq.sort_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0)));
        for i in 0..26 {
            if i < 25 && (freq[i + 1].1 - freq[i].1) * (i + 1) <= cq {
                cq -= (freq[i + 1].1 - freq[i].1) * (i + 1);
                if cq == 0 {
                    for j in 0..=i {
                        freq[j].1 = freq[i + 1].1;
                    }
                    break;
                }
                let save = freq[i + 1];
                let mut j = i + 1;
                while j > 0 {
                    j -= 1;
                    if freq[j].0 > save.0 {
                        freq[j + 1] = freq[j];
                    } else {
                        j += 1;
                        break;
                    }
                }
                freq[j] = save;
                freq[i + 1].1 = save.1;
            } else {
                let k = cq / (i + 1);
                let mut l = cq % (i + 1);
                for j in 0..=i {
                    freq[j].1 = freq[i].1 + k;
                    if l > 0 {
                        freq[j].1 += 1;
                        l -= 1;
                    }
                }
                break;
            }
        }
        freq.sort();
        let mut ic = 0;
        let mut ans = String::new();
        for &c in ss {
            if c == b'?' {
                while cnt[ic] == freq[ic].1 {
                    ic += 1;
                }
                ans.push((freq[ic].0 as u8 + b'a') as char);
                cnt[ic] += 1;
            } else {
                ans.push(c as char);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimize_string_value() {
        assert_eq!(
            Solution::minimize_string_value("abcdefghijklmnopqrstuvwxy??".to_string()),
            "abcdefghijklmnopqrstuvwxyaz".to_string()
        );
        assert_eq!(Solution::minimize_string_value("???".to_string()), "abc".to_string());
        assert_eq!(Solution::minimize_string_value("a?a?".to_string()), "abac".to_string());
    }
}

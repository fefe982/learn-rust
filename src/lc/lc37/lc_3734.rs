// https://leetcode.com/problems/lexicographically-smallest-palindromic-permutation-greater-than-target/
// 3734. Lexicographically Smallest Palindromic Permutation Greater Than Target
pub struct Solution;
impl Solution {
    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let target = target.chars().collect::<Vec<_>>();
        let mut count = [0; 26];
        for c in s.iter() {
            count[*c as usize - 'a' as usize] += 1;
        }
        let len = s.len();
        let mut mid = ' ';
        for i in 0..26 {
            if count[i] % 2 != 0 {
                if len % 2 == 0 || mid != ' ' {
                    return "".to_string();
                }
                mid = (i as u8 + b'a') as char;
            }
            count[i] /= 2;
        }
        let mut r = Vec::with_capacity(len);
        for i in 0..len / 2 {
            let ic = target[i] as usize - 'a' as usize;
            if count[ic] > 0 {
                r.push(target[i]);
                count[ic] -= 1;
            } else {
                for j in ic + 1..26 {
                    if count[j] > 0 {
                        r.push((j as u8 + b'a') as char);
                        count[j] -= 1;
                        for k in 0..26 {
                            for _ in 0..count[k] {
                                r.push((k as u8 + b'a') as char);
                            }
                        }
                        if len % 2 == 1 {
                            r.push(mid);
                        }
                        for j in 0..len / 2 {
                            let c = r[len / 2 - 1 - j];
                            r.push(c);
                        }
                        return r.into_iter().collect();
                    }
                }
                break;
            }
        }
        if r.len() == len / 2 {
            let mut res = r.clone();
            let mut cmp = 0;
            if len % 2 == 1 {
                if mid > target[len / 2] {
                    cmp = 1;
                } else if mid < target[len / 2] {
                    cmp = -1;
                }
                res.push(mid);
            }
            if cmp >= 0 {
                let llen = len - len / 2;
                for i in 0..len / 2 {
                    let c = r[len / 2 - 1 - i];
                    if cmp == 0 {
                        let tc = target[llen + i];
                        if c > tc {
                            cmp = 1;
                        } else if c < tc {
                            cmp = -1;
                            break;
                        }
                    }
                    res.push(c);
                }
            }
            if cmp > 0 {
                return res.into_iter().collect();
            }
        }
        'nextperm: while let Some(c) = r.pop() {
            let ic = c as usize - 'a' as usize;
            count[ic] += 1;
            for i in ic + 1..26 {
                if count[i] > 0 {
                    r.push((i as u8 + b'a') as char);
                    count[i] -= 1;
                    break 'nextperm;
                }
            }
        }
        if r.is_empty() {
            return "".to_string();
        }
        for k in 0..26 {
            for _ in 0..count[k] {
                r.push((k as u8 + b'a') as char);
            }
        }
        if len % 2 == 1 {
            r.push(mid);
        }
        for j in 0..len / 2 {
            let c = r[len / 2 - 1 - j];
            r.push(c);
        }
        r.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lex_palindromic_permutation() {
        assert_eq!(
            Solution::lex_palindromic_permutation("aaaabbbb".to_string(), "abbbaabba".to_string()),
            "baabbaab".to_string()
        );
        assert_eq!(
            Solution::lex_palindromic_permutation("aabb".to_string(), "baaa".to_string()),
            "baab".to_string()
        );
        assert_eq!(
            Solution::lex_palindromic_permutation("aabb".to_string(), "abaa".to_string()),
            "abba".to_string()
        );
        assert_eq!(
            Solution::lex_palindromic_permutation("baba".to_string(), "abba".to_string()),
            "baab".to_string()
        );
        assert_eq!(
            Solution::lex_palindromic_permutation("baba".to_string(), "bbaa".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::lex_palindromic_permutation("abc".to_string(), "abb".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::lex_palindromic_permutation("aac".to_string(), "abb".to_string()),
            "aca".to_string()
        );
    }
}

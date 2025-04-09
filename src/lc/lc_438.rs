// https://leetcode.com/problems/find-all-anagrams-in-a-string/
// 438. Find All Anagrams in a String
pub struct Solution;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut cntp = vec![0; 26];
        let mut cnts = vec![0; 26];
        let plen = p.len();
        let mut cnt = 0;
        for c in p.as_bytes() {
            let idx = (c - b'a') as usize;
            cntp[idx] += 1;
            if cntp[idx] == 1 {
                cnt += 1;
            }
        }
        let mut res = Vec::with_capacity(s.len());
        let mut cs = 0;
        for (i, c) in s.as_bytes().iter().enumerate() {
            let idx = (c - b'a') as usize;
            cnts[idx] += 1;
            if cnts[idx] == cntp[idx] {
                cs += 1;
            }
            if i >= plen {
                let idx = (s.as_bytes()[i - plen] - b'a') as usize;
                cnts[idx] -= 1;
                if cnts[idx] == cntp[idx] - 1 {
                    cs -= 1;
                }
            }
            if cs == cnt {
                res.push((i + 1 - plen) as i32);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_anagrams() {
        assert_eq!(
            Solution::find_anagrams(
                "dinitrophenylhydrazinetrinitrophenylmethylnitramine".to_string(),
                "trinitrophenylmethylnitramine".to_string()
            ),
            [19, 20, 21, 22]
        );
        assert_eq!(
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
            [0, 6]
        );
        assert_eq!(Solution::find_anagrams("abab".to_string(), "ab".to_string()), [0, 1, 2]);
    }
}

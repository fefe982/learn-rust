// https://leetcode.com/problems/palindrome-partitioning/
// 131. Palindrome Partitioning
pub struct Solution;
impl Solution {
    fn palindrome(s: &[u8], i: usize, j: usize, pal: &mut Vec<Vec<i8>>) -> bool {
        if pal[i][j] >= 0 {
            return pal[i][j] == 1;
        }
        let mut ii = i;
        let mut jj = j;
        while ii < jj && s[ii] == s[jj] {
            ii += 1;
            jj -= 1;
            if pal[ii][jj] == 1 {
                for k in i..ii {
                    pal[k][j - k + i] = 1;
                }
                return true;
            }
            if pal[ii][jj] == 0 {
                for k in i..ii {
                    pal[k][j - k + i] = 0;
                }
                return false;
            }
        }
        if ii < jj {
            while ii > i {
                pal[ii][jj] = 0;
                ii -= 1;
                jj += 1;
            }
            pal[i][j] = 0;
            return false;
        }
        while ii > i {
            ii -= 1;
            jj += 1;
            pal[ii][jj] = 1;
        }
        return true;
    }
    fn collect_partition(s: &[u8], i: usize, pal: &mut Vec<Vec<i8>>, part: &mut Vec<Vec<Vec<String>>>) {
        if !part[i].is_empty() {
            return;
        }
        let mut v = vec![];
        for j in i..s.len() {
            if Self::palindrome(s, i, j, pal) {
                let sp = String::from_utf8(s[i..=j].to_vec()).unwrap();
                if j + 1 == s.len() {
                    v.push(vec![sp]);
                } else {
                    Self::collect_partition(s, j + 1, pal, part);
                    for p in part[j + 1].iter() {
                        v.push(vec![sp.clone()]);
                        v.last_mut().unwrap().extend_from_slice(p);
                    }
                }
            }
        }
        part[i] = v;
    }
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.as_bytes();
        let mut pal = vec![vec![-1; s.len()]; s.len()];
        for i in 0..s.len() {
            pal[i][i] = 1;
        }
        let mut part = vec![vec![]; s.len()];
        Self::collect_partition(s, 0, &mut pal, &mut part);
        part[0].clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_partition() {
        assert_eq!(
            Solution::partition("aab".to_string()),
            vec_vec_str![["a", "a", "b"], ["aa", "b"]]
        );
        assert_eq!(Solution::partition("a".to_string()), vec_vec_str![["a"]]);
    }
}

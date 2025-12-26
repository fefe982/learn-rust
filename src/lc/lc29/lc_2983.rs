// https://leetcode.com/problems/palindrome-rearrangement-queries/
// 2983. Palindrome Rearrangement Queries
pub struct Solution;
impl Solution {
    pub fn can_make_palindrome_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let s = s.as_bytes();
        let n = s.len() / 2;
        let mut diff = vec![0; n + 1];
        let mut cnt = vec![vec![vec![0; 26]; 2]; n + 1];
        for i in 0..n {
            diff[i + 1] = diff[i];
            cnt[i + 1] = cnt[i].clone();
            if s[i] != s[n + n - i - 1] {
                diff[i + 1] += 1;
            }
            cnt[i + 1][0][(s[i] - b'a') as usize] += 1;
            cnt[i + 1][1][(s[n + n - i - 1] - b'a') as usize] += 1;
        }
        if cnt[n][0] != cnt[n][1] {
            return vec![false; queries.len()];
        }
        let ndiff = diff[n];
        let mut ans = vec![];
        for q in queries {
            let (l1, r1, l2, r2) = (
                q[0] as usize,
                q[1] as usize,
                n + n - q[3] as usize - 1,
                n + n - q[2] as usize - 1,
            );
            if diff[l1.min(l2)] > 0 {
                ans.push(false);
                continue;
            }
            if diff[r1.max(r2) + 1] != ndiff {
                ans.push(false);
                continue;
            }
            let lo = l1.max(l2);
            let ro = r1.min(r2);
            if lo > ro {
                if diff[lo] - diff[ro + 1] > 0 {
                    ans.push(false);
                    continue;
                }
            }
            let mut good = true;
            for i in 0..26 {
                let co = if ro >= lo {
                    ((cnt[ro + 1][0][i] - cnt[lo][0][i]), (cnt[ro + 1][1][i] - cnt[lo][1][i]))
                } else {
                    (0, 0)
                };
                if cnt[r1 + 1][1][i] - cnt[l1][1][i] - co.1 > cnt[r1 + 1][0][i] - cnt[l1][0][i] {
                    good = false;
                    break;
                }
                if cnt[r2 + 1][0][i] - cnt[l2][0][i] - co.0 > cnt[r2 + 1][1][i] - cnt[l2][1][i] {
                    good = false;
                    break;
                }
            }
            ans.push(good);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_can_make_palindrome_queries() {
        assert_eq!(
            Solution::can_make_palindrome_queries("natvhtruvwyutyvvnarhwt".to_string(), vec_vec![[0, 8, 11, 19]]),
            [false]
        );
        assert_eq!(
            Solution::can_make_palindrome_queries("bbccbb".to_string(), vec_vec![[0, 1, 4, 5]]),
            [true]
        );
        assert_eq!(
            Solution::can_make_palindrome_queries("abcabc".to_string(), vec_vec![[1, 1, 3, 5], [0, 2, 5, 5]]),
            [true, true]
        );
        assert_eq!(
            Solution::can_make_palindrome_queries("abbcdecbba".to_string(), vec_vec![[0, 2, 7, 9]]),
            [false]
        );
        assert_eq!(
            Solution::can_make_palindrome_queries("acbcab".to_string(), vec_vec![[1, 2, 4, 5]]),
            [true]
        );
    }
}

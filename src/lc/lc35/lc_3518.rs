// https://leetcode.cn/problems/smallest-palindromic-rearrangement-ii/
// 3518. Smallest Palindromic Rearrangement II
pub struct Solution;
impl Solution {
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let mut cnt = [0i64; 26];
        let s = s.as_bytes();
        for i in 0..s.len() / 2 {
            cnt[(s[i] - b'a') as usize] += 1;
        }
        let mut perm = 1;
        let mut tot = 0;
        let mut cnt_s = [0; 26];
        let k = k as i64;
        'out: for i in (0..26).rev() {
            if cnt[i] == 0 {
                continue;
            }
            for j in 1..=cnt[i] {
                tot += 1;
                perm = perm * tot / j;
                cnt[i] -= 1;
                cnt_s[i] += 1;
                if perm >= k {
                    break 'out;
                }
            }
        }
        if perm < k {
            return "".to_string();
        }
        let mut ans = Vec::with_capacity(s.len());
        for i in 0..26 {
            for _ in 0..cnt[i] {
                ans.push((i as u8 + b'a') as char);
            }
        }
        let mut t = k;
        while tot > 0 {
            for i in 0..26 {
                if cnt_s[i] > 0 {
                    let c = perm * cnt_s[i] / tot;
                    if c >= t {
                        ans.push((i as u8 + b'a') as char);
                        cnt_s[i] -= 1;
                        tot -= 1;
                        perm = c;
                        break;
                    } else {
                        t -= c;
                    }
                }
            }
        }
        if s.len() % 2 == 1 {
            ans.push(s[s.len() / 2] as char);
        }
        for i in (0..s.len() / 2).rev() {
            ans.push(ans[i]);
        }
        ans.into_iter().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_palindrome() {
        assert_eq!(Solution::smallest_palindrome("abba".to_string(), 2), "baab");
        assert_eq!(Solution::smallest_palindrome("aa".to_string(), 2), "");
        assert_eq!(Solution::smallest_palindrome("bacab".to_string(), 1), "abcba");
    }
}

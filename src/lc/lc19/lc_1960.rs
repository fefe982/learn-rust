// https://leetcode.com/problems/maximum-product-of-the-length-of-two-palindromic-substrings/
// 1960. Maximum Product of the Length of Two Palindromic Substrings
pub struct Solution;
impl Solution {
    pub fn max_product(s: String) -> i64 {
        let s = s.as_bytes();
        let n = s.len();
        let mut c = vec![0; n];
        let mut l = 0;
        let mut r = 0;
        for i in 1..n {
            if i < r {
                c[i] = c[l + r - i].min(r - i);
            } else {
                c[i] = 0;
            }
            if i + c[i] >= r {
                while i + c[i] + 1 < n && i >= c[i] + 1 && s[i + c[i] + 1] == s[i - c[i] - 1] {
                    c[i] += 1;
                }
                if i + c[i] > r {
                    r = i + c[i];
                    l = i - c[i];
                }
            }
        }
        let mut q = std::collections::VecDeque::new();
        let mut rlength = vec![0; n];
        q.push_back((n - 1, c[n - 1]));
        rlength[n - 1] = 1;
        for j in (1..n - 1).rev() {
            q.push_back((j, c[j]));
            while let Some(&(i, length)) = q.front() {
                if j < i - length {
                    q.pop_front();
                } else {
                    rlength[j] = (2 * (i - j) + 1).max(rlength[j + 1]);
                    break;
                }
            }
        }
        q.clear();
        let mut ans = 0;
        let mut max_left = 0;
        for j in 0..n - 1 {
            q.push_back((j, c[j]));
            while let Some(&(i, length)) = q.front() {
                if j > i + length {
                    q.pop_front();
                } else {
                    max_left = max_left.max(2 * (j - i) + 1);
                    ans = ans.max(rlength[j + 1] as i64 * max_left as i64);
                    break;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_product() {
        assert_eq!(Solution::max_product("ab".to_owned()), 1);
        assert_eq!(Solution::max_product("ababbb".to_owned()), 9);
        assert_eq!(Solution::max_product("zaaaxbbby".to_owned()), 9);
    }
}

// https://leetcode.com/problems/find-the-closest-palindrome/
// 564. Find the Closest Palindrome
pub struct Solution;
impl Solution {
    fn get_rev(mut n: i64, even: bool) -> i64 {
        let mut nc = n;
        if !even {
            nc /= 10;
        }
        while nc > 0 {
            n *= 10;
            n += nc % 10;
            nc /= 10;
        }
        n
    }
    pub fn nearest_palindromic(n: String) -> String {
        let l = n.as_bytes().len();
        let n = n.parse::<i64>().unwrap();
        let mut nh = 1;
        for _ in 0..l / 2 {
            nh *= 10;
        }
        let n2 = n / nh;
        let mut n9 = nh * nh;
        let mut n10 = n9;
        if l % 2 == 1 {
            n10 *= 10;
            nh *= 10;
        } else {
            n9 /= 10;
        }
        let mut cand = vec![n10 + 1, n10 - 1, n9 + 1, n9 - 1];
        cand.push(Self::get_rev(n2, l % 2 == 0));
        if n2 != nh * 10 - 1 {
            cand.push(Self::get_rev(n2 + 1, l % 2 == 0));
        }
        if n2 != nh {
            cand.push(Self::get_rev(n2 - 1, l % 2 == 0));
        }
        let mut d = i64::MAX;
        let mut r = 0;
        for c in cand {
            if c != n && ((c - n).abs() < d || ((c - n).abs() == d && c < r)) {
                r = c;
                d = (c - n).abs();
            }
        }
        r.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nearest_palindromic() {
        assert_eq!(Solution::nearest_palindromic("123".to_string()), "121".to_string());
        assert_eq!(Solution::nearest_palindromic("1".to_string()), "0".to_string());
    }
}

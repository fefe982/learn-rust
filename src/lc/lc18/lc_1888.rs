// https://leetcode.com/problems/minimum-number-of-flips-to-make-the-binary-string-alternating/
// 1888. Minimum Number of Flips to Make the Binary String Alternating
pub struct Solution;
impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let s = s.as_bytes();
        let len = s.len();
        let mut even = vec![0; len + 1];
        let mut odd = vec![0; len + 1];
        for i in 0..len {
            if i % 2 == 0 {
                even[i + 1] = even[i] + (s[i] == b'1') as i32;
                odd[i + 1] = odd[i];
            } else {
                even[i + 1] = even[i];
                odd[i + 1] = odd[i] + (s[i] == b'1') as i32;
            }
        }
        if len % 2 == 0 {
            (len as i32 / 2 - even[len] + odd[len]).min(len as i32 / 2 - odd[len] + even[len])
        } else {
            let mut eb = 0;
            let mut ob = 0;
            let lo = len as i32 / 2;
            let le = len as i32 - lo;
            let mut min = i32::MAX;
            for i in (0..len).rev() {
                if i % 2 == 0 {
                    eb += (s[i] == b'1') as i32;
                } else {
                    ob += (s[i] == b'1') as i32;
                }
                let fo = i as i32 / 2;
                let fe = i as i32 - fo;
                min = min.min(
                    (fe - even[i] + odd[i]).min(fo - odd[i] + even[i]) + (lo - fo - ob + eb).min(le - fe - eb + ob),
                );
            }
            min
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_flips() {
        assert_eq!(Solution::min_flips("111000".to_string()), 2);
        assert_eq!(Solution::min_flips("010".to_string()), 0);
        assert_eq!(Solution::min_flips("1110".to_string()), 1);
    }
}

// https://leetcode.com/problems/replace-the-substring-for-balanced-string/
// 1234. Replace the Substring for Balanced String
pub struct Solution;
impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut cnt = [0; 4];
        let mut l = 0i32;
        fn get_idx(c: char) -> usize {
            match c {
                'Q' => 0,
                'W' => 1,
                'E' => 2,
                'R' => 3,
                _ => 4,
            }
        }
        for &c in &s {
            cnt[get_idx(c)] += 1;
            l += 1;
        }
        l /= 4;
        let mut c = cnt
            .iter()
            .filter_map(|&x| if x > l { Some(1) } else { None })
            .sum::<i32>();
        if c == 0 {
            return 0;
        }
        let mut j = 0;
        let mut min = usize::MAX;
        for i in 0..s.len() {
            loop {
                if c == 0 {
                    min = min.min(j - i);
                    break;
                }
                if j == s.len() {
                    break;
                }
                let idx = get_idx(s[j]);
                cnt[idx] -= 1;
                if cnt[idx] == l {
                    c -= 1;
                }
                j += 1;
            }
            let idx = get_idx(s[i]);
            cnt[idx] += 1;
            if cnt[idx] == l + 1 {
                c += 1;
            }
            if c > 0 && j == s.len() {
                break;
            }
        }
        min as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn balanced_string() {
        assert_eq!(Solution::balanced_string("WWEQERQWQWWRWWERQWEQ".to_string()), 4);
        assert_eq!(Solution::balanced_string("QWER".to_string()), 0);
        assert_eq!(Solution::balanced_string("QQWE".to_string()), 1);
        assert_eq!(Solution::balanced_string("QQQW".to_string()), 2);
    }
}

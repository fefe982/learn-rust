// https://leetcode.com/problems/minimum-operations-to-make-character-frequencies-equal/
// 3389. Minimum Operations to Make Character Frequencies Equal
pub struct Solution;
impl Solution {
    pub fn make_string_good(s: String) -> i32 {
        let mut cnt = [0; 26];
        let s = s.as_bytes();
        let mut ans = s.len() as i32;
        for c in s {
            cnt[(c - b'a') as usize] += 1;
        }
        let mc: i32 = *cnt.iter().max().unwrap();
        for t in 1..=mc {
            let mut keep = [0; 26];
            let mut throw = [0; 26];
            keep[0] = (cnt[0] - t).abs();
            throw[0] = cnt[0];
            for i in 1..26 {
                let last = keep[i - 1].min(throw[i - 1]);
                throw[i] = last + cnt[i];
                if t > cnt[i] {
                    let need = t - cnt[i];
                    keep[i] = (keep[i - 1] + (need - (cnt[i - 1] - t).max(0)).max(0))
                        .min(throw[i - 1] + (need - cnt[i - 1]).max(0));
                } else {
                    keep[i] = last + cnt[i] - t;
                }
            }
            ans = ans.min(keep[25].min(throw[25]));
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_string_good() {
        assert_eq!(Solution::make_string_good("gigigjjggjjgg".to_string()), 3);
        assert_eq!(Solution::make_string_good("acab".to_string()), 1);
        assert_eq!(Solution::make_string_good("wddw".to_string()), 0);
        assert_eq!(Solution::make_string_good("aaabc".to_string()), 2);
    }
}

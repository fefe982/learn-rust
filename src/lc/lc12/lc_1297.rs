// https://leetcode.com/problems/maximum-number-of-occurrences-of-a-substring/
// 1297. Maximum Number of Occurrences of a Substring
pub struct Solution;
impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, _max_size: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut max = 0;
        let mut cnt = vec![0; 26];
        let mut cur = 0;
        let mut sum = 0i128;
        let min_size = min_size as usize;
        let mut mul = 1i128;
        let s = s.as_bytes();
        for (i, &c) in s.iter().enumerate() {
            let idx = (c - b'a') as usize;
            if cnt[idx] == 0 {
                cur += 1;
            }
            cnt[idx] += 1;
            sum = sum * 26 + idx as i128;
            if i + 1 >= min_size {
                if cur <= max_letters {
                    let e = map.entry(sum).or_insert(0);
                    *e += 1;
                    max = max.max(*e);
                }
                let idx0 = (s[i + 1 - min_size] - b'a') as usize;
                sum -= idx0 as i128 * mul;
                cnt[idx0] -= 1;
                if cnt[idx0] == 0 {
                    cur -= 1;
                }
            } else {
                mul *= 26;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_freq() {
        assert_eq!(Solution::max_freq("aababcaab".to_string(), 2, 3, 4), 2);
        assert_eq!(Solution::max_freq("aaaa".to_string(), 1, 3, 3), 2);
    }
}

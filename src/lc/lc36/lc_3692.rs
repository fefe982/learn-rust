// https://leetcode.com/problems/majority-frequency-characters/
// 3692. Majority Frequency Characters
pub struct Solution;
impl Solution {
    pub fn majority_frequency_group(s: String) -> String {
        let mut cnt = [0; 26];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        let mut freq = std::collections::BTreeMap::<i32, std::collections::BTreeSet<char>>::new();
        for i in 0..26 {
            if cnt[i] > 0 {
                freq.entry(cnt[i]).or_default().insert((i as u8 + b'a') as char);
            }
        }
        let mut ans = String::new();
        let mut max = 0;
        for (_, v) in freq.iter().rev() {
            if v.len() > max {
                max = v.len();
                ans = v.iter().collect();
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn majority_frequency_group() {
        assert_eq!(
            Solution::majority_frequency_group("aaabbbccdddde".to_string()),
            "ab".to_string()
        );
        assert_eq!(
            Solution::majority_frequency_group("abcd".to_string()),
            "abcd".to_string()
        );
        assert_eq!(
            Solution::majority_frequency_group("pfpfgi".to_string()),
            "fp".to_string()
        );
    }
}

// https://leetcode.com/problems/maximize-the-confusion-of-an-exam/
// 2024. Maximize the Confusion of an Exam
pub struct Solution;
impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let mut max = 0;
        let mut cnt = vec![0, 0];
        let mut left = 0;
        let answer_key = answer_key.as_bytes();
        for (i, &c) in answer_key.iter().enumerate() {
            if c == b'T' {
                cnt[1] += 1;
            } else {
                cnt[0] += 1;
            }
            while cnt[0] > k && cnt[1] > k {
                if answer_key[left] == b'T' {
                    cnt[1] -= 1;
                } else {
                    cnt[0] -= 1;
                }
                left += 1;
            }
            max = max.max(i - left + 1);
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_consecutive_answers() {
        assert_eq!(Solution::max_consecutive_answers("TTFF".to_string(), 2), 4);
        assert_eq!(Solution::max_consecutive_answers("TFFT".to_string(), 1), 3);
        assert_eq!(Solution::max_consecutive_answers("TTFTTFTT".to_string(), 1), 5)
    }
}

// https://leetcode.com/problems/sum-of-scores-of-built-strings/
// 2223. Sum of Scores of Built Strings
pub struct Solution;
impl Solution {
    pub fn sum_scores(s: String) -> i64 {
        let s = s.as_bytes();
        let mut len = vec![0; s.len()];
        len[0] = s.len();
        let mut l = 0;
        let mut r = 1;
        for i in 1..s.len() {
            let start = if i >= r { i } else { i + len[i - l].min(r - i) };
            let mut end = false;
            for j in start..s.len() {
                if s[j] != s[j - i] {
                    if j > r {
                        l = i;
                        r = j;
                    }
                    len[i] = j - i;
                    end = true;
                    break;
                }
            }
            if !end {
                if r != s.len() {
                    l = i;
                    r = s.len();
                }
                len[i] = s.len() - i;
            }
        }
        len.into_iter().map(|x| x as i64).sum::<i64>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_scores() {
        assert_eq!(Solution::sum_scores("babab".to_string()), 9);
        assert_eq!(Solution::sum_scores("azbazbzaz".to_string()), 14);
    }
}

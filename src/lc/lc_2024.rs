// https://leetcode.com/problems/maximize-the-confusion-of-an-exam/
// 2024. Maximize the Confusion of an Exam
pub struct Solution;
impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let mut tmap = std::collections::BTreeMap::<i32, i32>::new();
        tmap.insert(0, 0);
        let mut fmap = tmap.clone();
        let (mut nt, mut nf) = (0, 0);
        let mut max = 0;
        for (i, c) in (1..).zip(answer_key.chars()) {
            if c == 'T' {
                nt += 1;
                tmap.insert(nt, i);
            } else {
                nf += 1;
                fmap.insert(nf, i);
            }
            if nt <= k || nf <= k {
                max = i;
                continue;
            }
            max = max
                .max(i - *tmap.get(&(nt - k)).unwrap())
                .max(i - *fmap.get(&(nf - k)).unwrap());
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_consecutive_answers() {
        assert_eq!(Solution::max_consecutive_answers("TTFF".to_string(), 2), 4);
        assert_eq!(Solution::max_consecutive_answers("TFFT".to_string(), 1), 3);
        assert_eq!(
            Solution::max_consecutive_answers("TTFTTFTT".to_string(), 1),
            5
        )
    }
}

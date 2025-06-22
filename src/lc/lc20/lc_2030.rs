// https://leetcode.com/problems/smallest-k-length-subsequence-with-occurrences-of-a-letter/
// 2030. Smallest K-Length Subsequence With Occurrences of a Letter
pub struct Solution;
impl Solution {
    pub fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let len = s.len();
        let cnt = s.iter().filter(|&c| *c == letter).count();
        let mut res = vec![];
        let mut nletter = 0;
        let mut nletter_left = cnt;
        let k = k as usize;
        let repetition = repetition as usize;
        for (i, c) in s.into_iter().enumerate() {
            while let Some(&last) = res.last() {
                if last <= c {
                    break;
                }
                if res.len() - 1 + len - i >= k
                    && nletter + nletter_left - if last == letter { 1 } else { 0 } >= repetition
                {
                    res.pop();
                    nletter -= if last == letter { 1 } else { 0 };
                } else {
                    break;
                }
            }
            if res.len() < k && (c == letter || res.len() + 1 - nletter + repetition <= k) {
                res.push(c);
                if c == letter {
                    nletter += 1;
                }
            }
            if letter == c {
                nletter_left -= 1;
            }
        }
        res.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smallest_subsequence() {
        assert_eq!(Solution::smallest_subsequence("leet".to_string(), 3, 'e', 1), "eet");
        assert_eq!(
            Solution::smallest_subsequence("leetcode".to_string(), 4, 'e', 2),
            "ecde"
        );
        assert_eq!(Solution::smallest_subsequence("bb".to_string(), 2, 'b', 2), "bb");
    }
}

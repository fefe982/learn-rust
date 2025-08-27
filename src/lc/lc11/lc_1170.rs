// https://leetcode.com/problems/compare-strings-by-frequency-of-the-smallest-character/
// 1170. Compare Strings by Frequency of the Smallest Character
pub struct Solution;
impl Solution {
    pub fn count(word: String) -> i32 {
        let mut min_count = 0;
        let mut low_char = b'z';
        for &c in word.as_bytes() {
            if c < low_char {
                low_char = c;
                min_count = 1;
            } else if c == low_char {
                min_count += 1;
            }
        }
        min_count
    }
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut val = vec![0; 11];
        for w in words {
            val[Self::count(w) as usize - 1] += 1;
        }
        for i in (0..10).rev() {
            val[i] += val[i + 1];
        }
        queries
            .into_iter()
            .map(|q| val[Self::count(q) as usize])
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn num_smaller_by_frequency() {
        assert_eq!(
            Solution::num_smaller_by_frequency(vec_str!["cbd"], vec_str!["zaaaz"]),
            vec![1]
        );
        assert_eq!(
            Solution::num_smaller_by_frequency(
                vec_str!["bbb", "cc"],
                vec_str!["a", "aa", "aaa", "aaaa"]
            ),
            vec![1, 2]
        );
    }
}

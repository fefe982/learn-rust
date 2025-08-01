// https://leetcode.com/problems/count-vowel-strings-in-ranges/
// 2559. Count Vowel Strings in Ranges
pub struct Solution;
impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowel = std::collections::HashSet::<u8>::from([b'a', b'e', b'i', b'o', b'u']);
        let mut arr = vec![0; words.len() + 1];
        for (i, w) in words.iter().enumerate() {
            if vowel.contains(&w.as_bytes()[0]) && vowel.contains(&w.as_bytes()[w.len() - 1]) {
                arr[i + 1] = arr[i] + 1;
            } else {
                arr[i + 1] = arr[i];
            }
        }
        let mut res = Vec::new();
        for q in queries {
            res.push(arr[q[1] as usize + 1] - arr[q[0] as usize]);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn vowel_strings() {
        assert_eq!(
            Solution::vowel_strings(
                vec_str!["aba", "bcb", "ece", "aa", "e"],
                vec_vec![[0, 2], [1, 4], [1, 1]]
            ),
            vec![2, 3, 0]
        );
        assert_eq!(
            Solution::vowel_strings(vec_str!["a", "e", "i"], vec_vec![[0, 2], [0, 1], [2, 2]]),
            vec![3, 2, 1]
        );
    }
}

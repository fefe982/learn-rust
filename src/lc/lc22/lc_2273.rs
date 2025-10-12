// https://leetcode.com/problems/find-resultant-array-after-removing-anagrams/
// 2273. Find Resultant Array After Removing Anagrams
pub struct Solution;
impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut res = vec![];
        let mut last_sort = vec![];
        for word in words {
            let mut sorted = word.chars().collect::<Vec<char>>();
            sorted.sort_unstable();
            if last_sort != sorted {
                res.push(word);
                last_sort = sorted;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn remove_anagrams() {
        assert_eq!(
            Solution::remove_anagrams(vec_str!["abba", "baba", "bbaa", "cd", "cd"]),
            vec_str!["abba", "cd"]
        );
        assert_eq!(
            Solution::remove_anagrams(vec_str!["a", "b", "c", "d", "e"]),
            vec_str!["a", "b", "c", "d", "e"]
        );
    }
}

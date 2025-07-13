// https://leetcode.com/problems/maximum-score-words-formed-by-letters/
// 1255. Maximum Score Words Formed by Letters
pub struct Solution;
impl Solution {
    fn sum(words: &Vec<(Vec<usize>, i32)>, letter_map: &mut std::collections::HashMap<usize, i32>, i: usize) -> i32 {
        if i == words.len() {
            return 0;
        }
        let (word, score) = &words[i];
        let mut good = true;
        for &l in word {
            let cnt = letter_map.entry(l).or_default();
            *cnt -= 1;
            if *cnt < 0 {
                good = false;
            }
        }
        let mut ans = 0;
        if good {
            ans = *score + Solution::sum(words, letter_map, i + 1);
        }
        for &l in word {
            *letter_map.entry(l).or_default() += 1;
        }
        ans.max(Solution::sum(words, letter_map, i + 1))
    }
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let words = words
            .into_iter()
            .map(|x| {
                let x = x.as_bytes().iter().map(|x| (x - b'a') as usize).collect::<Vec<_>>();
                let s = x.iter().fold(0, |acc, x| acc + score[*x]);
                (x, s)
            })
            .collect::<Vec<_>>();
        let mut letter_map = std::collections::HashMap::<usize, i32>::new();
        for l in letters {
            *letter_map.entry((l as u8 - b'a') as usize).or_default() += 1;
        }
        Solution::sum(&words, &mut letter_map, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_score_words() {
        assert_eq!(
            Solution::max_score_words(
                vec_str!["dog", "cat", "dad", "good"],
                vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
                vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ),
            23
        );
        assert_eq!(
            Solution::max_score_words(
                vec_str!["xxxz", "ax", "bx", "cx"],
                vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'],
                vec![4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10]
            ),
            27
        );
        assert_eq!(
            Solution::max_score_words(
                vec_str!["leetcode"],
                vec_chr!["l", "e", "t", "c", "o", "d"],
                vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
            ),
            0
        );
    }
}

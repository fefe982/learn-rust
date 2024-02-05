// https://leetcode.com/problems/number-of-valid-words-for-each-puzzle/
// 1178. Number of Valid Words for Each Puzzle
pub struct Solution;
impl Solution {
    pub fn mask_work(word: &str) -> i32 {
        let mut m = 0;
        for c in word.as_bytes() {
            m |= 1 << (c - b'a');
        }
        m
    }
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut ans = vec![];
        let mut w_map = std::collections::HashMap::<i32, i32>::new();
        for word in words {
            let mask = Self::mask_work(&word);
            *w_map.entry(mask).or_default() += 1;
        }
        for p in puzzles {
            let mut mp = Self::mask_work(&p);
            let first = 1 << (p.as_bytes()[0] - b'a');
            mp -= first;
            let mut submask = mp;
            let mut cnt = 0;
            loop {
                cnt += w_map.get(&(submask + first)).unwrap_or(&0);
                if submask == 0 {
                    break;
                }
                submask = mp & (submask - 1);
            }
            ans.push(cnt);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_num_of_valid_words() {
        assert_eq!(
            Solution::find_num_of_valid_words(
                vec_str!["aaaa", "asas", "able", "ability", "actt", "actor", "access"],
                vec_str!["aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz"]
            ),
            vec![1, 1, 3, 2, 4, 0]
        );
        assert_eq!(
            Solution::find_num_of_valid_words(
                vec_str!["apple", "pleas", "please"],
                vec_str!["aelwxyz", "aelpxyz", "aelpsxy", "saelpxy", "xaelpsy"]
            ),
            vec![0, 1, 3, 2, 0]
        );
    }
}

// https://leetcode.com/problems/find-smallest-letter-greater-than-target/
// 744. Find Smallest Letter Greater Than Target
pub struct Solution;
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let pos = letters.partition_point(|&c| c <= target);
        if pos < letters.len() {
            letters[pos]
        } else {
            letters[0]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn next_greatest_letter() {
        assert_eq!(
            Solution::next_greatest_letter(vec_chr!["c", "f", "j"], 'a'),
            'c'
        );
        assert_eq!(
            Solution::next_greatest_letter(vec_chr!["c", "f", "j"], 'c'),
            'f'
        );
        assert_eq!(
            Solution::next_greatest_letter(vec_chr!["x", "x", "y", "y"], 'z'),
            'x'
        );
    }
}

// https://leetcode.com/problems/letter-case-permutation/
// 784. Letter Case Permutation
pub struct Solution;
impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut mask: i32 = 0;
        let s = s
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_ascii_digit() {
                    c
                } else {
                    let b = c.to_ascii_lowercase();
                    mask |= 1 << i;
                    b
                }
            })
            .collect::<String>();
        let mut m = mask;
        let mut res = Vec::with_capacity(1 << mask.count_ones());
        while m != 0 {
            let t = s
                .chars()
                .enumerate()
                .map(|(i, c)| if m & (1 << i) != 0 { c.to_ascii_uppercase() } else { c })
                .collect::<String>();
            res.push(t);
            m = (m - 1) & mask;
        }
        res.push(s);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn letter_case_permutation() {
        assert_sort_eq!(
            Solution::letter_case_permutation("a1b2".to_string()),
            vec!["a1b2", "a1B2", "A1b2", "A1B2"]
        );
        assert_sort_eq!(Solution::letter_case_permutation("3z4".to_string()), vec!["3z4", "3Z4"]);
    }
}

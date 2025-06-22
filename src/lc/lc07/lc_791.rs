// https://leetcode.com/problems/custom-sort-string/
// 791. Custom Sort String
pub struct Solution;
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut ord = vec![0; 26];
        for (i, c) in order.chars().enumerate() {
            ord[(c as u8 - b'a') as usize] = i;
        }
        let mut cv = s.chars().collect::<Vec<_>>();
        cv.sort_unstable_by_key(|&c| ord[(c as u8 - b'a') as usize]);
        cv.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    fn check_result(order: &str, input: &str, output: &str) {
        let im = input.chars().counts();
        let om = output.chars().counts();
        assert!(im == om);
        let ord = order
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i))
            .collect::<std::collections::HashMap<char, usize>>();
        let mut last = 0;
        for c in output.chars() {
            if let Some(&o) = ord.get(&c) {
                assert!(o >= last);
                last = o;
            }
        }
    }
    #[test]
    fn test_custom_sort_string() {
        for (ord, s) in [("cba", "abcd"), ("cbafg", "abcd")] {
            check_result(ord, s, &Solution::custom_sort_string(ord.to_string(), s.to_string()));
        }
    }
}

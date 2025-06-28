// https://leetcode.com/problems/number-of-lines-to-write-string/
// 806. Number of Lines to Write String
pub struct Solution;
impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut lines = 1;
        let mut current_width = 0;
        for c in s.chars() {
            let width = widths[c as usize - 'a' as usize];
            if current_width + width > 100 {
                lines += 1;
                current_width = width;
            } else {
                current_width += width;
            }
        }
        vec![lines, current_width]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_lines() {
        assert_eq!(
            Solution::number_of_lines(
                vec![
                    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10
                ],
                "abcdefghijklmnopqrstuvwxyz".to_string()
            ),
            vec![3, 60]
        );
        assert_eq!(
            Solution::number_of_lines(
                vec![
                    4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                    10
                ],
                "bbbcccdddaaa".to_string()
            ),
            vec![2, 4]
        );
    }
}

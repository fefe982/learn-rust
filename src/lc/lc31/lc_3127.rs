// https://leetcode.com/problems/make-a-square-with-the-same-color/
// 3127. Make a Square With the Same Color
pub struct Solution;
impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        let mut last2 = vec![];
        let mut last = vec![0; grid[0].len() + 1];
        for row in grid {
            let mut cur = vec![0];
            let mut cb = 0;
            for (i, c) in row.into_iter().enumerate() {
                if c == 'B' {
                    cb += 1;
                }
                cur.push(cb);
                cur[i + 1] += last[i + 1];
                if i > 0 && last2.len() > 0 {
                    if cur[i + 1] + last2[i - 1] - cur[i - 1] - last2[i + 1] != 2 {
                        return true;
                    }
                }
            }
            last2 = last;
            last = cur;
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_can_make_square() {
        assert_eq!(
            Solution::can_make_square(vec_vec_chr![["B", "W", "B"], ["B", "W", "W"], ["B", "W", "B"]]),
            true
        );
        assert_eq!(
            Solution::can_make_square(vec_vec_chr![["B", "W", "B"], ["W", "B", "W"], ["B", "W", "B"]]),
            false
        );
        assert_eq!(
            Solution::can_make_square(vec_vec_chr![["B", "W", "B"], ["B", "W", "W"], ["B", "W", "W"]]),
            true
        );
    }
}

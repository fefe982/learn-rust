// https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/
// 1893. Check if All the Integers in a Range Are Covered
pub struct Solution;
impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut covered = [false; 51];
        for range in ranges {
            for x in range[0]..=range[1] {
                covered[x as usize] = true;
            }
        }
        (left..=right).all(|x| covered[x as usize])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_covered() {
        assert_eq!(Solution::is_covered(vec_vec![[1, 2], [3, 4], [5, 6]], 2, 5), true);
        assert_eq!(Solution::is_covered(vec_vec![[1, 10], [10, 20]], 21, 21), false);
    }
}

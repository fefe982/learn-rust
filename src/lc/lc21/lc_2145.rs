// https://leetcode.cn/problems/count-the-hidden-sequences/
// 2145. Count the Hidden Sequences
pub struct Solution;
impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut min = 0;
        let mut max = 0;
        let mut sum = 0;
        for d in differences {
            sum += d;
            min = min.min(sum);
            max = max.max(sum);
            if max - min > upper - lower {
                return 0;
            }
        }
        upper - lower - max + min + 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_of_arrays() {
        assert_eq!(Solution::number_of_arrays(vec![1, -3, 4], 1, 6), 2);
        assert_eq!(Solution::number_of_arrays(vec![3, -4, 5, 1, -2], -4, 5), 4);
    }
}

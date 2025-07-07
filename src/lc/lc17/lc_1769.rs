// https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/
// 1769. Minimum Number of Operations to Move All Balls to Each Box
pub struct Solution;
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes = boxes.as_bytes();
        let mut res = vec![0; boxes.len()];
        let mut cntf = 0;
        let mut stepf = 0;
        let mut cntb = 0;
        let mut stepb = 0;
        let len = boxes.len();
        for i in 0..len {
            res[i] += cntf;
            if boxes[i] == b'1' {
                stepf += 1;
            }
            cntf += stepf;
            res[len - i - 1] += cntb;
            if boxes[len - i - 1] == b'1' {
                stepb += 1;
            }
            cntb += stepb;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations("110".to_string()), [1, 1, 3]);
        assert_eq!(Solution::min_operations("001011".to_string()), [11, 8, 5, 4, 3, 4]);
    }
}

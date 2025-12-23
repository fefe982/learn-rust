// https://leetcode.com/problems/apple-redistribution-into-boxes/
// 3074. Apple Redistribution Into Boxes
pub struct Solution;
impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut sum = apple.into_iter().sum::<i32>();
        let mut capacity = capacity;
        capacity.sort_unstable();
        let mut res = 0;
        for c in capacity.into_iter().rev() {
            res += 1;
            sum -= c;
            if sum <= 0 {
                return res;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_boxes() {
        assert_eq!(Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]), 2);
        assert_eq!(Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]), 4);
    }
}

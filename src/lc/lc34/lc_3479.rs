// https://leetcode.com/problems/fruits-into-baskets-iii/
// 3479. Fruits Into Baskets III
pub struct Solution;
impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        super::lc_3477::Solution::num_of_unplaced_fruits(fruits, baskets)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_of_unplaced_fruits() {
        assert_eq!(Solution::num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4]), 1);
        assert_eq!(Solution::num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7]), 0);
    }
}

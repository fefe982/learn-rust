// https://leetcode.com/problems/number-of-burgers-with-no-waste-of-ingredients/
// 1276. Number of Burgers with No Waste of Ingredients
pub struct Solution;
impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        if tomato_slices % 2 == 1 || tomato_slices / 2 < cheese_slices || tomato_slices / 4 > cheese_slices {
            return vec![];
        }
        vec![tomato_slices / 2 - cheese_slices, 2 * cheese_slices - tomato_slices / 2]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_of_burgers() {
        assert_eq!(Solution::num_of_burgers(16, 7), vec![1, 6]);
        assert_eq!(Solution::num_of_burgers(17, 4), vec![]);
        assert_eq!(Solution::num_of_burgers(4, 17), vec![]);
    }
}

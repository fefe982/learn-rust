// https://leetcode.com/problems/optimal-division/
// 553. Optimal Division
pub struct Solution;
impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        if nums.len() == 1 {
            nums[0].to_string()
        } else if nums.len() == 2 {
            format!("{}/{}", nums[0], nums[1])
        } else {
            format!(
                "{}/({})",
                nums[0],
                nums[1..].iter().map(|x| x.to_string()).collect::<Vec<_>>().join("/")
            )
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn optimal_division() {
        assert_eq!(Solution::optimal_division(vec![2]), "2");
        assert_eq!(Solution::optimal_division(vec![1000, 100, 10, 2]), "1000/(100/10/2)");
        assert_eq!(Solution::optimal_division(vec![2, 3, 4]), "2/(3/4)");
    }
}

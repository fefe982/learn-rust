// https://leetcode.com/problems/construct-the-rectangle/
// 492. Construct the Rectangle
pub struct Solution;
impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let s = (area as f64).sqrt() as i32;
        for i in (1..=s).rev() {
            if area % i == 0 {
                return vec![area / i, i];
            }
        }
        vec![]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct_rectangle() {
        assert_eq!(Solution::construct_rectangle(4), vec![2, 2]);
        assert_eq!(Solution::construct_rectangle(37), vec![37, 1]);
        assert_eq!(Solution::construct_rectangle(122122), vec![427, 286]);
    }
}

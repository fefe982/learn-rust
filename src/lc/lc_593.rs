// https://leetcode.com/problems/valid-square/
// 593. Valid Square
pub struct Solution;
impl Solution {
    fn check(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>) -> bool {
        if p1 == p2
            || (p1[0] - p2[0]) * (p1[0] - p2[0]) + (p1[1] - p2[1]) * (p1[1] - p2[1])
                != (p1[0] - p3[0]) * (p1[0] - p3[0]) + (p1[1] - p3[1]) * (p1[1] - p3[1])
        {
            return false;
        }
        (p1[0] - p2[0]) * (p1[0] - p3[0]) + (p1[1] - p2[1]) * (p1[1] - p3[1]) == 0
    }
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut v = [p1, p2, p3, p4];
        v.sort();
        Self::check(&v[0], &v[1], &v[2])
            && Self::check(&v[1], &v[0], &v[3])
            && Self::check(&v[2], &v[0], &v[3])
            && Self::check(&v[3], &v[1], &v[2])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_square() {
        assert_eq!(
            Solution::valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1]),
            true
        );
        assert_eq!(
            Solution::valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 12]),
            false
        );
        assert_eq!(
            Solution::valid_square(vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]),
            true
        );
    }
}

// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
// 1365. How Many Numbers Are Smaller Than the Current Number
pub struct Solution;
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; nums.len()];
        let mut n = nums.into_iter().enumerate().collect::<Vec<_>>();
        n.sort_by(|a, b| a.1.cmp(&b.1));
        let mut c = 0;
        let mut cc = 1;
        let mut l = n[0].1;
        for i in 1..n.len() {
            if l == n[i].1 {
                cc += 1;
                count[n[i].0] = c;
            } else {
                c += cc;
                cc = 1;
                l = n[i].1;
                count[n[i].0] = c;
            }
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smaller_numbers_than_current() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        )
    }
}

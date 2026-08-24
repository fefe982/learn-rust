// https://leetcode.com/problems/smallest-missing-multiple-of-k/description/?envType=daily-question&envId=2026-08-25
// 3718. Smallest Missing Multiple of K
pub struct Solution;
impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let mut mul = vec![false; 102];
        let mut nm = 1;
        for n in nums {
            if n % k == 0 {
                mul[(n / k) as usize] = true;
            }
            while mul[nm] {
                nm += 1;
            }
        }
        nm as i32 * k
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_multiple() {
        assert_eq!(Solution::missing_multiple(vec![8, 2, 3, 4, 6], 2), 10);
        assert_eq!(Solution::missing_multiple(vec![1, 4, 7, 10, 15], 5), 5);
    }
}

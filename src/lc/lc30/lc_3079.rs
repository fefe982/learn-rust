// https://leetcode.com/problems/find-the-sum-of-encrypted-integers/
// 3079. Find the Sum of Encrypted Integers
pub struct Solution;
impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for n in nums {
            let mut one = 0;
            let mut maxd = 0;
            let mut nn = n;
            while nn > 0 {
                maxd = maxd.max(nn % 10);
                nn /= 10;
                one = one * 10 + 1;
            }
            sum += maxd * one;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_of_encrypted_int() {
        assert_eq!(Solution::sum_of_encrypted_int(vec![1, 2, 3]), 6);
        assert_eq!(Solution::sum_of_encrypted_int(vec![10, 21, 31]), 66);
    }
}

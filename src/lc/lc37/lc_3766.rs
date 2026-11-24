// https://leetcode.com/problems/minimum-operations-to-make-binary-palindrome/
// 3766. Minimum Operations to Make Binary Palindrome
pub struct Solution;
impl Solution {
    fn pal(n: i32, odd: bool) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut r = n;
        let mut l = n;
        if odd {
            l /= 2;
        }
        while l > 0 {
            r *= 2;
            if l % 2 == 1 {
                r += 1;
            }
            l /= 2;
        }
        r
    }
    pub fn min_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(nums.len());
        for n in nums {
            if n < 2 {
                ans.push(0);
                continue;
            }
            let bit = i32::BITS - n.leading_zeros();
            let odd = bit % 2 == 1;
            let nn = n >> (bit / 2);
            let mut min = i32::MAX;
            min = min.min((Self::pal(nn, odd) - n).abs());
            min = min.min((Self::pal(nn - 1, odd) - n).abs());
            min = min.min((Self::pal(nn + 1, odd) - n).abs());
            ans.push(min);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![1, 2, 4]), vec![0, 1, 1]);
        assert_eq!(Solution::min_operations(vec![6, 7, 12]), vec![1, 0, 3]);
    }
}

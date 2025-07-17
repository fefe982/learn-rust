// https://leetcode.com/problems/replace-non-coprime-numbers-in-array/
// 2197. Replace Non-Coprime Numbers in Array
pub struct Solution;
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        loop {
            if b == 0 {
                return a;
            }
            a %= b;
            if a == 0 {
                return b;
            }
            b %= a;
        }
    }
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        for n in nums {
            let mut next = n;
            while let Some(&last) = ans.last() {
                let g = Solution::gcd(last, next);
                if g == 1 {
                    break;
                }
                ans.pop();
                next = next / g * last;
            }
            ans.push(next);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_replace_non_coprimes() {
        assert_eq!(Solution::replace_non_coprimes(vec![6, 4, 3, 2, 7, 6, 2]), [12, 7, 6]);
        assert_eq!(Solution::replace_non_coprimes(vec![2, 2, 1, 1, 3, 3, 3]), [2, 1, 1, 3]);
    }
}

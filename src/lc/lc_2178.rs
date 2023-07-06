// https://leetcode.com/problems/maximum-split-of-positive-even-integers/
// 2178. Maximum Split of Positive Even Integers
pub struct Solution;
impl Solution {
    pub fn maximum_even_split(mut final_sum: i64) -> Vec<i64> {
        let mut r = vec![];
        if final_sum % 2 == 0 {
            let mut n = 2;
            while final_sum - n > n {
                r.push(n);
                final_sum -= n;
                n += 2;
            }
            r.push(final_sum);
        }
        r
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_even_split() {
        assert_eq!(Solution::maximum_even_split(12), vec![2, 4, 6]);
        assert_eq!(Solution::maximum_even_split(7), Vec::<i64>::new());
        assert_eq!(Solution::maximum_even_split(28), vec![2, 4, 6, 16]);
    }
}

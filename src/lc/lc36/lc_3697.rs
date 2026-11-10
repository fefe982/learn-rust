// https://leetcode.com/problems/compute-decimal-representation/
// 3697. Compute Decimal Representation
pub struct Solution;
impl Solution {
    pub fn decimal_representation(n: i32) -> Vec<i32> {
        let mut n = n;
        let mut res = Vec::new();
        let mut p = 1;
        while n > 0 {
            if n % 10 > 0 {
                res.push(p * (n % 10));
            }
            p *= 10;
            n /= 10;
        }
        res.reverse();
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decimal_representation() {
        assert_eq!(Solution::decimal_representation(537), vec![500, 30, 7]);
        assert_eq!(Solution::decimal_representation(102), vec![100, 2]);
        assert_eq!(Solution::decimal_representation(6), vec![6]);
    }
}

// https://leetcode.com/problems/non-negative-integers-without-consecutive-ones/
// 600. Non-negative Integers without Consecutive Ones

pub struct Solution;
impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let mut n0 = vec![1; 32];
        n0[1] = 2;
        for i in 2..32 {
            n0[i] = n0[i - 1] + n0[i - 2];
        }
        let mut last = 0;
        let mut sum = 0;
        for i in (0..31).rev() {
            let current = n & (1 << i);
            if current > 0 {
                sum += n0[i];
                if last > 0 {
                    return sum;
                }
            }
            last = current
        }
        sum + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_integers() {
        assert_eq!(Solution::find_integers(5), 5);
        assert_eq!(Solution::find_integers(1), 2);
        assert_eq!(Solution::find_integers(2), 3);
    }
}

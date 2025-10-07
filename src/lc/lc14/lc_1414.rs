// https://leetcode.com/problems/find-the-minimum-number-of-fibonacci-numbers-whose-sum-is-k/
// 1414. Find the Minimum Number of Fibonacci Numbers Whose Sum Is K
pub struct Solution;
impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut fibs = vec![1, 1];
        while *fibs.last().unwrap() < k {
            let n = fibs.len();
            fibs.push(fibs[n - 1] + fibs[n - 2]);
        }
        let mut count = 0;
        let mut remain = k;
        for &f in fibs.iter().rev() {
            if f <= remain {
                remain -= f;
                count += 1;
            }
            if remain == 0 {
                break;
            }
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_min_fibonacci_numbers() {
        assert_eq!(Solution::find_min_fibonacci_numbers(7), 2);
        assert_eq!(Solution::find_min_fibonacci_numbers(10), 2);
        assert_eq!(Solution::find_min_fibonacci_numbers(19), 3);
    }
}

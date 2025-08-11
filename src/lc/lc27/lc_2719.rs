// https://leetcode.com/problems/count-of-integers/
// 2719. Count of Integers
pub struct Solution;
const MOD: i64 = 1_0000_0000_7i64;
impl Solution {
    fn count0(
        num2: &[i32],
        min_sum: i32,
        mut max_sum: i32,
        cache: &mut std::collections::HashMap<(i32, i32, i32), i64>,
    ) -> i64 {
        if max_sum < 0 {
            return 0;
        }
        if min_sum >= 9 * num2.len() as i32 + 1 {
            return 0;
        }
        if num2.len() == 0 {
            if min_sum == 0 {
                return 1;
            } else {
                return 0;
            }
        }
        max_sum = max_sum.min(9 * num2.len() as i32);
        let mut sum = 0;
        for (idx, &d2) in num2.iter().enumerate().rev() {
            if 0 != d2 {
                for d in 0..d2 {
                    sum += Self::count09(idx as i32, (min_sum - d).max(0), max_sum - d, cache);
                }
                sum += Self::count0(&num2[..idx], (min_sum - d2).max(0), max_sum - d2, cache);
                sum %= MOD;
                break;
            }
            if idx == 0 && min_sum == 0 {
                sum += 1;
            }
        }
        sum %= MOD;
        sum
    }
    fn count9(
        num1: &[i32],
        mut min_sum: i32,
        mut max_sum: i32,
        cache: &mut std::collections::HashMap<(i32, i32, i32), i64>,
    ) -> i64 {
        if max_sum < 0 {
            return 0;
        }
        if min_sum >= 9 * num1.len() as i32 + 1 {
            return 0;
        }
        if num1.len() == 0 {
            if min_sum == 0 {
                return 1;
            } else {
                return 0;
            }
        }
        max_sum = max_sum.min(9 * num1.len() as i32);
        let mut sum = 0;
        for (idx, &d1) in num1.iter().enumerate().rev() {
            if d1 != 9 {
                sum = Self::count9(&num1[..idx], (min_sum - d1).max(0), max_sum - d1, cache);
                for d in d1 + 1..10 {
                    sum += Self::count09(idx as i32, (min_sum - d).max(0), max_sum - d, cache);
                }
                sum %= MOD;
                break;
            }
            min_sum = (min_sum - 9).max(0);
            max_sum -= 9;
            if max_sum < 0 {
                break;
            }
            if idx == 0 && min_sum == 0 {
                sum += 1;
            }
        }
        sum %= MOD;
        sum
    }
    fn count09(
        digit: i32,
        min_sum: i32,
        mut max_sum: i32,
        cache: &mut std::collections::HashMap<(i32, i32, i32), i64>,
    ) -> i64 {
        if max_sum < 0 {
            return 0;
        }
        if min_sum >= 9 * digit + 1 {
            return 0;
        }
        if digit == 0 {
            return 1;
        }
        if digit == 1 {
            return (max_sum.min(9) - min_sum + 1) as i64;
        }
        max_sum = max_sum.min(9 * digit as i32);
        if let Some(&v) = cache.get(&(digit, min_sum, max_sum)) {
            return v;
        }
        let mut sum = 0;
        for d in 0..10 {
            sum += Self::count09(digit - 1, (min_sum - d).max(0), max_sum - d, cache);
        }
        sum %= MOD;
        cache.insert((digit, min_sum, max_sum), sum);
        sum
    }
    fn count1(num1: &[i32], num2: &[i32], mut min_sum: i32, mut max_sum: i32) -> i32 {
        if max_sum < 0 {
            return 0;
        }
        let mut cache = std::collections::HashMap::<(i32, i32, i32), i64>::new();
        let mut sum = 0i64;
        for (idx, (&d1, &d2)) in num1.iter().zip(num2.iter()).enumerate().rev() {
            if d1 != d2 {
                sum = Self::count9(&num1[..idx], (min_sum - d1).max(0), max_sum - d1, &mut cache);
                for d in d1 + 1..d2 {
                    sum += Self::count09(idx as i32, (min_sum - d).max(0), max_sum - d, &mut cache);
                }
                sum += Self::count0(&num2[..idx], (min_sum - d2).max(0), max_sum - d2, &mut cache);
                sum %= MOD;
                break;
            } else {
                min_sum = (min_sum - d1).max(0);
                max_sum -= d1;
                if max_sum < 0 {
                    break;
                }
            }
        }
        sum as i32
    }
    pub fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
        let mut num1 = num1.chars().map(|c| (c as u8 - b'0') as i32).rev().collect::<Vec<_>>();
        let num2 = num2.chars().map(|c| (c as u8 - b'0') as i32).rev().collect::<Vec<_>>();
        while num1.len() < num2.len() {
            num1.push(0);
        }
        Self::count1(&num1, &num2, min_sum, max_sum)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count() {
        assert_eq!(Solution::count("11".to_string(), "330".to_string(), 1, 100), 320);
        assert_eq!(Solution::count("11".to_string(), "330".to_string(), 1, 2), 6);
        assert_eq!(Solution::count("1".to_string(), "12".to_string(), 1, 8), 11);
        assert_eq!(Solution::count("1".to_string(), "5".to_string(), 1, 5), 5);
    }
}

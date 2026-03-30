// https://leetcode.com/problems/count-almost-equal-pairs-ii/
// 3267. Count Almost Equal Pairs II
pub struct Solution;
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn count_pairs(nums: Vec<i32>) -> i32 {
        let mut seen = HashMap::<i32, i32>::new();
        let mut ans = 0_i64;

        for num in nums {
            let reachable = Self::reachable_with_two_swaps(num);
            for val in reachable {
                ans += i64::from(*seen.get(&val).unwrap_or(&0));
            }
            *seen.entry(num).or_insert(0) += 1;
        }

        ans as i32
    }

    fn reachable_with_two_swaps(num: i32) -> HashSet<i32> {
        let base = Self::to_digits(num);
        let mut values = HashSet::new();
        values.insert(Self::digits_to_num(&base));

        for i in 0..7 {
            for j in i + 1..7 {
                let mut one = base;
                one.swap(i, j);
                values.insert(Self::digits_to_num(&one));

                for a in 0..7 {
                    for b in a + 1..7 {
                        let mut two = one;
                        two.swap(a, b);
                        values.insert(Self::digits_to_num(&two));
                    }
                }
            }
        }

        values
    }

    fn to_digits(mut num: i32) -> [u8; 7] {
        let mut digits = [0_u8; 7];
        for i in (0..7).rev() {
            digits[i] = (num % 10) as u8;
            num /= 10;
        }
        digits
    }

    fn digits_to_num(digits: &[u8; 7]) -> i32 {
        let mut val = 0_i32;
        for &d in digits {
            val = val * 10 + i32::from(d);
        }
        val
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_pairs() {
        assert_eq!(Solution::count_pairs(vec![1023, 2310, 2130, 213]), 4);
        assert_eq!(Solution::count_pairs(vec![1, 10, 100]), 3);
    }
}

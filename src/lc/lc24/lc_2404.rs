// https://leetcode.com/problems/most-frequent-even-element/
// 2404. Most Frequent Even Element
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();
        for n in nums {
            if n % 2 == 0 {
                *cnt.entry(n).or_default() += 1;
            }
        }
        let mut max = -1;
        let mut max_cnt = -1;
        for (n, c) in cnt {
            if c > max_cnt {
                max_cnt = c;
                max = n;
            } else if c == max_cnt && n < max {
                max = n;
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn most_frequent_even() {
        assert_eq!(Solution::most_frequent_even(vec![0, 1, 2, 2, 4, 4, 1]), 2);
        assert_eq!(Solution::most_frequent_even(vec![4, 4, 4, 9, 2, 4]), 4);
        assert_eq!(
            Solution::most_frequent_even(vec![29, 47, 21, 41, 13, 37, 25, 7]),
            -1
        );
    }
}

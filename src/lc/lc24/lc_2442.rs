// https://leetcode.com/problems/count-number-of-distinct-integers-after-reverse-operations/
// 2442. Count Number of Distinct Integers After Reverse Operations
pub struct Solution;
impl Solution {
    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        for &n in &nums {
            set.insert(n);
            let mut n = n;
            let mut rev = 0;
            while n > 0 {
                rev = rev * 10 + n % 10;
                n /= 10;
            }
            set.insert(rev);
        }
        set.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_distinct_integers() {
        assert_eq!(Solution::count_distinct_integers(vec![1, 13, 10, 12, 31]), 6);
        assert_eq!(Solution::count_distinct_integers(vec![2, 2, 2]), 1);
    }
}

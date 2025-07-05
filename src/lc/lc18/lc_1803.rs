// https://leetcode.com/problems/count-pairs-with-xor-in-a-range/
// 1803. Count Pairs With XOR in a Range
pub struct Solution;
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32 {
        let mut cnt = std::collections::HashMap::<i32, i32>::new();
        for num in nums {
            *cnt.entry(num).or_default() += 1;
        }
        let mut ans = 0;
        let mut low = low;
        let mut high = high + 1;
        while low > 0 || high > 0 {
            let mut ncnt = std::collections::HashMap::<i32, i32>::new();
            for (&n, &cn) in &cnt {
                *ncnt.entry(n >> 1).or_default() += cn;
                if high & 1 == 1 {
                    ans += cn * cnt.get(&(n ^ (high - 1))).unwrap_or(&0);
                }
                if low & 1 == 1 {
                    ans -= cn * (cnt.get(&(n ^ (low - 1)))).unwrap_or(&0);
                }
            }
            high >>= 1;
            low >>= 1;
            cnt = ncnt;
        }
        ans / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_pairs() {
        assert_eq!(Solution::count_pairs(vec![1, 4, 2, 7], 2, 6), 6);
        assert_eq!(Solution::count_pairs(vec![9, 8, 4, 2, 1], 5, 14), 8);
    }
}

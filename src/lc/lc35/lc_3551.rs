// https://leetcode.com/problems/minimum-swaps-to-sort-by-digit-sum/
// 3551. Minimum Swaps to Sort by Digit Sum
pub struct Solution;
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut vnums = nums
            .into_iter()
            .enumerate()
            .map(|(i, x)| {
                let mut sum = 0;
                let mut n = x;
                while n > 0 {
                    sum += n % 10;
                    n /= 10;
                }
                (sum, x, i)
            })
            .collect::<Vec<_>>();
        vnums.sort();
        let mut ans = 0;
        for i in 0..vnums.len() {
            let mut j = vnums[i].2;
            while j != i {
                let k = vnums[j].2;
                vnums[j].2 = j;
                vnums[i].2 = k;
                j = k;
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_swaps() {
        assert_eq!(Solution::min_swaps(vec![37, 100]), 1);
        assert_eq!(Solution::min_swaps(vec![22, 14, 33, 7]), 0);
        assert_eq!(Solution::min_swaps(vec![18, 43, 34, 16]), 2);
    }
}

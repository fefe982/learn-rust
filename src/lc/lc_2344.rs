// https://leetcode.com/problems/minimum-deletions-to-make-array-divisible/
// 2344. Minimum Deletions to Make Array Divisible
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, nums_divide: Vec<i32>) -> i32 {
        let gcd = nums_divide
            .into_iter()
            .reduce(|mut a: i32, mut b: i32| loop {
                if b == 0 {
                    return a;
                }
                a = a % b;
                if a == 0 {
                    return b;
                }
                b = b % a;
            })
            .unwrap();
        let mut cnt = 0;
        let mut nums = nums
            .into_iter()
            .filter_map(|n| if n <= gcd { Some(std::cmp::Reverse(n)) } else { None })
            .collect::<std::collections::BinaryHeap<_>>();
        while let Some(std::cmp::Reverse(n)) = nums.pop() {
            if gcd % n == 0 {
                return cnt;
            } else {
                cnt += 1;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![2, 3, 2, 4, 3], vec![9, 6, 9, 3, 15]), 2);
        assert_eq!(Solution::min_operations(vec![4, 3, 6], vec![8, 2, 6, 10]), -1);
    }
}

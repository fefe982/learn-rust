// https://leetcode.com/problems/minimum-operations-to-make-array-equal-to-target/
// 3229. Minimum Operations to Make Array Equal to Target
pub struct Solution;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let mut sum = 0;
        let mut last = 0;
        for (a, b) in nums.into_iter().zip(target) {
            let d = b - a;
            match last.cmp(&0) {
                std::cmp::Ordering::Equal => {
                    sum += d.abs() as i64;
                }
                std::cmp::Ordering::Less => {
                    if d < last {
                        sum += (last - d) as i64;
                    } else if d > 0 {
                        sum += d as i64;
                    }
                }
                std::cmp::Ordering::Greater => {
                    if d > last {
                        sum += (d - last) as i64;
                    } else if d < 0 {
                        sum -= d as i64;
                    }
                }
            }
            last = d;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_operations() {
        assert_eq!(Solution::minimum_operations(vec![3, 5, 1, 2], vec![4, 6, 2, 4]), 2);
        assert_eq!(Solution::minimum_operations(vec![1, 3, 2], vec![2, 1, 4]), 5);
    }
}

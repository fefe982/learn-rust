// https://leetcode.com/problems/the-number-of-beautiful-subsets/
// 2597. The Number of Beautiful Subsets
pub struct Solution;
impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = std::collections::HashMap::<i32, (i32, i32, i32, i32)>::new();
        let mut nums = nums;
        nums.sort_unstable();
        for num in nums {
            let c = num % k;
            if let Some((last, with, without, base)) = cnt.get_mut(&c) {
                let nwithout = if num > *last { *with + *without } else { *without };
                let nwith = if num - *last > k {
                    *with + *without
                } else if num - *last == k {
                    *without
                } else {
                    *with * 2 + *base
                };
                let nbase = if num - *last > 0 { *without } else { *base };
                *with = nwith;
                *without = nwithout;
                *base = nbase;
                *last = num;
            } else {
                cnt.insert(c, (num, 1, 1, 1));
            }
        }
        let mut sum = 1;
        for (_, (_, with, without, _)) in cnt {
            sum *= with + without;
        }
        sum - 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_beautiful_subsets() {
        assert_eq!(
            Solution::beautiful_subsets(
                vec![
                    1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000,
                    1000, 1000, 1000, 1000
                ],
                1
            ),
            1048575
        );
        assert_eq!(Solution::beautiful_subsets(vec![2, 4, 6], 2), 4);
        assert_eq!(Solution::beautiful_subsets(vec![1], 1), 1);
    }
}

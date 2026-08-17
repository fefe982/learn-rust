// https://leetcode.com/problems/find-the-largest-almost-missing-integer/
// 3471. Find the Largest Almost-Integer
pub struct Solution;
impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            let mut m = std::collections::BTreeMap::new();
            for n in nums {
                *m.entry(n).or_insert(0) += 1;
            }
            for (n, c) in m.iter().rev() {
                if *c == 1 {
                    return *n;
                }
            }
            -1
        } else if k == nums.len() as i32 {
            *nums.iter().max().unwrap()
        } else {
            let len = nums.len();
            let n0 = nums[0];
            let nn = nums[len - 1];
            if n0 == nn {
                return -1;
            }
            let mut c0 = 0;
            let mut cn = 0;
            for n in nums {
                if n == n0 {
                    c0 += 1;
                } else if n == nn {
                    cn += 1;
                }
            }
            if c0 == 1 && (n0 > nn || cn > 1) {
                n0
            } else if cn == 1 {
                nn
            } else {
                -1
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_integer() {
        assert_eq!(Solution::largest_integer(vec![7, 5, 9, 10, 0, 12, 3, 12, 10], 1), 9);
        assert_eq!(Solution::largest_integer(vec![7, 3, 4, 3, 0], 3), 7);
        assert_eq!(Solution::largest_integer(vec![3, 9, 2, 1, 7], 3), 7);
        assert_eq!(Solution::largest_integer(vec![3, 9, 7, 2, 1, 7], 4), 3);
        assert_eq!(Solution::largest_integer(vec![0, 0], 1), -1);
        assert_eq!(Solution::largest_integer(vec![0, 0], 2), 0);
    }
}

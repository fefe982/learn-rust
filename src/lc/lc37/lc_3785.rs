// https://leetcode.com/problems/minimum-swaps-to-avoid-forbidden-values/
// 3785. Minimum Swaps to Avoid Forbidden Values
pub struct Solution;
impl Solution {
    pub fn min_swaps(nums: Vec<i32>, forbidden: Vec<i32>) -> i32 {
        let mut mn = 0;
        let mut mc = 0;
        let mut smn = 0;
        let mut smc = 0;
        for i in 0..nums.len() {
            let n = nums[i];
            let f = forbidden[i];
            if mc == 0 {
                mc += 1;
                mn = n;
            } else if n == mn {
                mc += 1;
            } else {
                mc -= 1;
            }
            if mc == 0 {
                mc += 1;
                mn = f
            } else if f == mn {
                mc += 1;
            } else {
                mc -= 1;
            }
            if n == f {
                if smc == 0 {
                    smc += 1;
                    smn = n;
                } else if n == smn {
                    smc += 1;
                } else {
                    smc -= 1;
                }
            }
        }
        mc = 0;
        smc = 0;
        let mut sc = 0;
        for i in 0..nums.len() {
            let n = nums[i];
            let f = forbidden[i];
            if n == mn {
                mc += 1;
            }
            if f == mn {
                mc += 1;
            }
            if n == f {
                sc += 1;
                if n == smn {
                    smc += 1;
                }
            }
        }
        if mc > nums.len() {
            -1
        } else {
            smc.max((sc + 1) / 2)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_swaps() {
        assert_eq!(Solution::min_swaps(vec![1, 2, 3], vec![3, 2, 1]), 1);
        assert_eq!(Solution::min_swaps(vec![4, 6, 6, 5], vec![4, 6, 5, 5]), 2);
        assert_eq!(Solution::min_swaps(vec![7, 7], vec![8, 7]), -1);
    }
}

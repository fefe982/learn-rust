// https://leetcode.com/problems/max-pair-sum-in-an-array/
// 2815. Maximum Pair Sum in an Array
pub struct Solution;
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut l = [[0, 0]; 10];
        let mut ans = -1;
        for n in nums {
            let mut md = 0;
            let mut nn = n;
            while nn > 0 {
                let d = nn % 10;
                md = md.max(d);
                nn /= 10;
            }
            let imd = md as usize;
            let mut nn = n;
            if l[imd][0] < n {
                let t = l[imd][0];
                l[imd][0] = n;
                nn = t;
            }
            if l[imd][1] < nn {
                l[imd][1] = nn;
            }
            if l[imd][1] > 0 {
                ans = ans.max(l[imd][0] + l[imd][1]);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_sum() {
        assert_eq!(Solution::max_sum(vec![112, 131, 411]), -1);
        assert_eq!(Solution::max_sum(vec![2536, 1613, 3366, 162]), 5902);
        assert_eq!(Solution::max_sum(vec![51, 71, 17, 24, 42]), 88)
    }
}

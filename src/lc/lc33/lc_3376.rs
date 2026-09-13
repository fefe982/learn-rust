// https://leetcode.com/problems/minimum-time-to-break-locks-i/
// 3376. Minimum Time to Break the Lock I
pub struct Solution;
impl Solution {
    pub fn find_minimum_time(strength: Vec<i32>, k: i32) -> i32 {
        let n = strength.len();
        let nn = 1 << n;
        let mut t = vec![i32::MAX; nn];
        t[0] = 0;
        for i in 1..nn {
            let mut imask = i;
            while imask > 0 {
                let ix = i.count_ones() as i32;
                let x = 1 + k * (ix - 1);
                let id = imask.trailing_zeros() as usize;
                let last = i ^ (1 << id);
                t[i] = t[i].min(t[last] + (strength[id] + x - 1) / x);
                imask ^= 1 << id
            }
        }
        t[nn - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_minimum_time() {
        assert_eq!(Solution::find_minimum_time(vec![3, 4, 1], 1), 4);
        assert_eq!(Solution::find_minimum_time(vec![2, 5, 4], 2), 5);
    }
}

// https://leetcode.com/problems/maximum-enemy-forts-that-can-be-captured/
// 2511. Maximum Enemy Forts That Can Be Captured
pub struct Solution;
impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let mut m = 0;
        let mut last1 = usize::MAX;
        let mut lastm = usize::MAX;
        for (i, v) in forts.into_iter().enumerate() {
            if v == 1 {
                if lastm != usize::MAX {
                    m = m.max(i - lastm - 1);
                }
                last1 = i;
                lastm = usize::MAX;
            } else if v == -1 {
                if last1 != usize::MAX {
                    m = m.max(i - last1 - 1);
                }
                lastm = i;
                last1 = usize::MAX;
            }
        }
        m as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capture_forts() {
        assert_eq!(
            Solution::capture_forts(vec![1, 0, 0, -1, 0, 0, -1, 0, 0, 1]),
            2
        );
        assert_eq!(Solution::capture_forts(vec![1, 0, 0, -1, 0, 0, 0, 0, 1]), 4);
        assert_eq!(Solution::capture_forts(vec![0, 0, 1, -1]), 0);
    }
}

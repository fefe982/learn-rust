// https://leetcode.cn/problems/oPs9Bm/
// LCP 47. 入场安检
pub struct Solution;
impl Solution {
    pub fn security_check(capacities: Vec<i32>, k: i32) -> i32 {
        let mut v = vec![0; k as usize + 1];
        v[0] = 1;
        for c in capacities {
            let c = c - 1;
            for i in (0..=k - c).rev() {
                v[(i + c) as usize] = (v[(i + c) as usize] + v[i as usize]) % 1000000007;
            }
        }
        v[k as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn security_check() {
        assert_eq!(
            Solution::security_check(
                vec![
                    1, 29, 2, 16, 11, 10, 27, 15, 2, 26, 27, 29, 26, 14, 7, 18, 21, 3, 27, 24, 5, 1, 13, 22, 14, 11, 1,
                    7, 1, 23, 25, 1, 27, 22, 17, 20, 4, 7, 4, 20, 21, 7, 28, 10, 9, 23, 6, 4, 16
                ],
                406
            ),
            757305257
        );
        assert_eq!(Solution::security_check(vec![2, 2, 3], 2), 2);
        assert_eq!(Solution::security_check(vec![3, 3], 3), 0);
        assert_eq!(Solution::security_check(vec![4, 3, 2, 2], 6), 2);
    }
}

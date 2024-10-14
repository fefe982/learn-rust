// https://leetcode.com/problems/super-egg-drop/
// 887. Super Egg Drop
pub struct Solution;
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let k = k as usize;
        let mut f = vec![1; k + 1];
        f[0] = 0;
        for cnt in 2.. {
            for i in (1..=k).rev() {
                f[i] += f[i - 1] + 1;
            }
            if f[k] >= n {
                return cnt;
            }
        }
        unreachable!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_super_egg_drop() {
        assert_eq!(Solution::super_egg_drop(1, 2), 2);
        assert_eq!(Solution::super_egg_drop(2, 6), 3);
        assert_eq!(Solution::super_egg_drop(3, 14), 4);
    }
}

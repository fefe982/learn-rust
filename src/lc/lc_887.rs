// https://leetcode.com/problems/super-egg-drop/
// 887. Super Egg Drop
pub struct Solution;
impl Solution {
    fn drop(k: i32, n: i32, cache: &mut std::collections::HashMap<(i32, i32), i32>) -> i32 {
        if k == 1 {
            return n;
        }
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if let Some(&v) = cache.get(&(k, n)) {
            return v;
        }
        let mut l = 1;
        let mut r = n;
        while l < r {
            let m = (l + r) / 2;
            let broken = Self::drop(k - 1, m - 1, cache);
            let not_broken = Self::drop(k, n - m, cache);
            if broken >= not_broken {
                r = m;
            } else {
                l = m + 1;
            }
        }
        let a = Self::drop(k - 1, l - 1, cache) + 1;
        cache.insert((k, n), a);
        a
    }
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let mut cache = std::collections::HashMap::new();
        Self::drop(k, n, &mut cache)
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

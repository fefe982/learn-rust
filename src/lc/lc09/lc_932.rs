// https://leetcode.com/problems/beautiful-array
// 932. Beautiful Array
pub struct Solution;
impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut res = (1..=n).collect::<Vec<i32>>();
        res.sort_by_cached_key(|&x| {
            let mut r = 0;
            let mut x = x;
            for _ in 0..10 {
                r <<= 1;
                if x & 1 == 1 {
                    r |= 1;
                }
                x >>= 1;
            }
            r
        });
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(n: i32) {
        let ans = Solution::beautiful_array(n);
        assert_eq!(ans.len(), n as usize);
        let mut m = std::collections::HashMap::<i32, usize>::new();
        for (idx, i) in ans.into_iter().enumerate() {
            assert!(!m.contains_key(&i));
            assert!(i >= 1 && i <= n);
            m.insert(i, idx);
        }
        for i in 1..=n {
            for j in 0..(n - i) / 2 {
                let jj = i + (j + 1) * 2;
                let kk = (i + jj) / 2;
                assert!((m[&i] < m[&kk] && m[&jj] < m[&kk]) || (m[&i] > m[&kk] && m[&jj] > m[&kk]));
            }
        }
    }
    #[test]
    fn beautiful_array() {
        check(4);
        check(5);
        check(1000);
    }
}

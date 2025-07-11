// https://leetcode.com/problems/painting-a-grid-with-three-different-colors/
// 1931. Painting a Grid With Three Different Colors
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        if m == 1 {
            let mut res = 1;
            let mut pow = 2;
            let mut n = n - 1;
            while n > 0 {
                if n & 1 != 0 {
                    res = (res * pow) % MOD;
                }
                pow = (pow * pow) % MOD;
                n >>= 1;
            }
            return (res * 3 % MOD) as i32;
        }
        let state = 1 << (m - 1);
        let mut matrix = vec![vec![]; state];
        let mut s = vec![vec![vec![]; 2]; state];
        for i in 0..state {
            s[i][0].push(0);
            s[i][1].push(1);
            for j in 0..m as usize - 1 {
                let s0 = s[i][0][j];
                let s1 = s[i][1][j];
                if i & (1 << j) != 0 {
                    s[i][0].push((s0 + 2) % 3);
                    s[i][1].push((s1 + 2) % 3);
                } else {
                    s[i][0].push((s0 + 1) % 3);
                    s[i][1].push((s1 + 1) % 3);
                }
            }
        }
        for i in 0..state {
            for j in 0..state {
                let mut diff = true;
                for k in 1..m as usize {
                    if s[i][0][k] == s[j][1][k] {
                        diff = false;
                        break;
                    }
                }
                if diff {
                    matrix[i].push(j);
                }
            }
        }
        let mut cnt = vec![1; state];
        for _ in 1..n {
            let mut ncnt = vec![0; state];
            for (istate, j) in matrix.iter().enumerate() {
                for &k in j {
                    ncnt[k] = (cnt[istate] + cnt[state - istate - 1] + ncnt[k]) % MOD;
                }
            }
            cnt = ncnt;
        }
        cnt.into_iter().fold(0, |acc, x| (acc + x * 3) % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_color_the_grid() {
        assert_eq!(Solution::color_the_grid(1, 9), 768);
        assert_eq!(Solution::color_the_grid(3, 3), 246);
        assert_eq!(Solution::color_the_grid(1, 1), 3);
        assert_eq!(Solution::color_the_grid(1, 2), 6);
        assert_eq!(Solution::color_the_grid(5, 5), 580986);
    }
}

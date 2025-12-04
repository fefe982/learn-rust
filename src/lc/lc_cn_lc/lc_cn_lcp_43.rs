// https://leetcode.cn/problems/Y1VbOX/
// LCP 43. 十字路口的交通
pub struct Solution;
impl Solution {
    pub fn traffic_command(directions: Vec<String>) -> i32 {
        fn index(c: char) -> i32 {
            match c {
                'E' => 0,
                'S' => 1,
                'W' => 2,
                'N' => 3,
                _ => -1,
            }
        }
        const L: i32 = 1;
        const D: i32 = 2;
        const R: i32 = 3;
        fn fail_adj(r: i32, l: i32) -> bool {
            ((r == L || r == D) && (l == L || l == D)) || (r == R && l == D)
        }
        fn fail_d(x: i32, y: i32) -> bool {
            (x == L && (y == R || y == D)) || ((x == D || x == R) && y == L)
        }
        fn fail(e: i32, s: i32, w: i32, n: i32) -> bool {
            fail_adj(e, s) || fail_adj(s, w) || fail_adj(w, n) || fail_adj(n, e) || fail_d(e, w) || fail_d(s, n)
        }
        let directions = directions
            .into_iter()
            .map(|s| s.chars().map(index).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let ne = directions[0].len();
        let ns = directions[1].len();
        let nw = directions[2].len();
        let nn = directions[3].len();
        let mut dp = vec![vec![vec![vec![i32::MAX; nn + 1]; nw + 1]; ns + 1]; ne + 1];
        dp[0][0][0][0] = 0;
        for je in 0..=ne {
            let de = if je > 0 { directions[0][je - 1] } else { -1 };
            for js in 0..=ns {
                let ds = if js > 0 { (directions[1][js - 1] + 3) % 4 } else { -1 };
                for jw in 0..=nw {
                    let dw = if jw > 0 { (directions[2][jw - 1] + 2) % 4 } else { -1 };
                    for jn in 0..=nn {
                        let dn = if jn > 0 { (directions[3][jn - 1] + 1) % 4 } else { -1 };
                        for p in 1..16 {
                            let (ee, ss, ww, nn) = (p & 1, (p >> 1) & 1, (p >> 2) & 1, (p >> 3) & 1);
                            if je < ee || js < ss || jw < ww || jn < nn {
                                continue;
                            }
                            if !fail(
                                if ee > 0 { de } else { -1 },
                                if ss > 0 { ds } else { -1 },
                                if ww > 0 { dw } else { -1 },
                                if nn > 0 { dn } else { -1 },
                            ) {
                                dp[je][js][jw][jn] = dp[je][js][jw][jn].min(dp[je - ee][js - ss][jw - ww][jn - nn] + 1);
                            }
                        }
                    }
                }
            }
        }
        dp[ne][ns][nw][nn]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        assert_eq!(Solution::traffic_command(vec_str!["S", "W", "N", "E"]), 2);
        assert_eq!(Solution::traffic_command(vec_str!["W", "N", "ES", "W"]), 2);
        assert_eq!(Solution::traffic_command(vec_str!["NS", "WE", "SE", "EW"]), 3);
    }
}

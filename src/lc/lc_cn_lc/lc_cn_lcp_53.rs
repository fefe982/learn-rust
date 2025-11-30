// https://leetcode.cn/problems/EJvmW4/
// LCP 53. 守护太空城
pub struct Solution;
impl Solution {
    pub fn defend_space_city(time: Vec<i32>, position: Vec<i32>) -> i32 {
        let mut e = vec![vec![false; 101]; 5];
        let mut mn = 0;
        let mut mt = 0;
        for (t, p) in time.into_iter().zip(position.into_iter()) {
            e[t as usize - 1][p as usize] = true;
            mn = mn.max(p);
            mt = mt.max(t);
        }
        mn += 1;
        let mn = mn as usize;
        let mt = mt as usize;
        let mut dp = vec![vec![i32::MAX / 4; mn * mt + 1]; 1 << mt];
        dp[0][0] = 0;
        for n in 0..mn {
            for t in 0..mt {
                let idp = n * mt + t;
                for mask in 0..(1 << mt) {
                    if mask & 1 == 1 {
                        dp[mask >> 1][idp + 1] = dp[mask >> 1][idp + 1].min(dp[mask][idp]);
                    } else {
                        let mut mask1 = mask;
                        let mut mask2 = mask;
                        for tt in t..mt {
                            let d = tt - t;
                            if mask & (1 << d) != 0 {
                                break;
                            }
                            mask1 >>= 1;
                            dp[mask1][idp + d + 1] = dp[mask1][idp + d + 1].min(dp[mask][idp] + d as i32 + 2);
                            mask2 = (mask2 >> 1) | (1 << (mt - 1));
                            dp[mask2][idp + d + 1] = dp[mask2][idp + d + 1].min(dp[mask][idp] + d as i32 + 3);
                        }
                        if !e[t][n] {
                            dp[mask >> 1][idp + 1] = dp[mask >> 1][idp + 1].min(dp[mask][idp]);
                        }
                    }
                }
            }
        }
        dp[0][mn * mt]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defend_space_city() {
        assert_eq!(Solution::defend_space_city(vec![1, 2, 1], vec![6, 3, 3]), 5);
        assert_eq!(Solution::defend_space_city(vec![1, 2, 1], vec![6, 3, 3]), 5);
        assert_eq!(
            Solution::defend_space_city(vec![1, 1, 1, 2, 2, 3, 5], vec![1, 2, 3, 1, 2, 1, 3]),
            9
        );
    }
}

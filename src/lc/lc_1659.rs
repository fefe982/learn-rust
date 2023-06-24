// https://leetcode.com/problems/maximize-grid-happiness/
// 1659. Maximize Grid Happiness
pub struct Solution;
impl Solution {
    fn dfs(
        m: usize,
        n: usize,
        mm: usize,
        nn: usize,
        mask: usize,
        ic: usize,
        ec: usize,
        dp: &mut Vec<Vec<Vec<Vec<Vec<i32>>>>>,
    ) -> i32 {
        if (ic == 0 && ec == 0) || mm == m {
            return 0;
        }
        if dp[mm][nn][mask][ic][ec] != -1 {
            return dp[mm][nn][mask][ic][ec];
        }
        let nn3 = Self::pow3(nn);
        let up = mask / nn3 % 3;
        let left = mask / Self::pow3(nn + 1) % 3;
        let inn = if nn == 0 { n - 1 } else { nn - 1 };
        let imm = if nn == 0 { mm + 1 } else { mm };
        let nneighbor = (up != 0) as i32 + (left != 0) as i32;
        let mask_clear = mask - up * nn3;
        let score = Self::dfs(m, n, imm, inn, mask_clear, ic, ec, dp)
            .max(if ic > 0 {
                Self::dfs(m, n, imm, inn, mask_clear + 1 * nn3, ic - 1, ec, dp) + 120
                    - nneighbor * 30
                    + Self::penalty(up, left)
            } else {
                0
            })
            .max(if ec > 0 {
                Self::dfs(m, n, imm, inn, mask_clear + 2 * nn3, ic, ec - 1, dp)
                    + 40
                    + nneighbor * 20
                    + Self::penalty(up, left)
            } else {
                0
            });
        dp[mm][nn][mask][ic][ec] = score;
        score
    }
    fn penalty(up: usize, left: usize) -> i32 {
        [[0, -30, 20], [-30, -60, -10], [20, -10, 40]][up][left]
    }
    fn pow3(n: usize) -> usize {
        [1, 3, 9, 27, 81, 243][n]
    }
    pub fn get_max_grid_happiness(
        m: i32,
        n: i32,
        introverts_count: i32,
        extroverts_count: i32,
    ) -> i32 {
        Self::dfs(
            m as usize,
            n as usize,
            0,
            n as usize - 1,
            0,
            introverts_count as usize,
            extroverts_count as usize,
            &mut vec![
                vec![
                    vec![
                        vec![
                            vec![-1; extroverts_count as usize + 1];
                            introverts_count as usize + 1
                        ];
                        Self::pow3(n as usize)
                    ];
                    n as usize
                ];
                m as usize
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_max_grid_happiness() {
        assert_eq!(Solution::get_max_grid_happiness(2, 3, 1, 2), 240);
        assert_eq!(Solution::get_max_grid_happiness(3, 1, 2, 1), 260);
        assert_eq!(Solution::get_max_grid_happiness(2, 2, 4, 0), 240);
    }
}

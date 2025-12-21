// https://leetcode.cn/problems/xun-bao/
// LCP 13. 寻宝
pub struct Solution;
impl Solution {
    pub fn minimal_steps(maze: Vec<String>) -> i32 {
        let maze = maze
            .into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut buttons = vec![];
        let mut stones = vec![];
        let mut start = (0, 0);
        let mut target = (0, 0);
        let ni = maze.len();
        let nj = maze[0].len();
        for i in 0..ni {
            for j in 0..nj {
                match maze[i][j] {
                    'M' => buttons.push((i, j)),
                    'O' => stones.push((i, j)),
                    'S' => start = (i, j),
                    'T' => target = (i, j),
                    _ => (),
                }
            }
        }
        let nb = buttons.len();
        let bfs = |start: (usize, usize)| {
            let mut dist = vec![vec![-1; nj]; ni];
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);
            dist[start.0][start.1] = 0;
            while let Some((pi, pj)) = queue.pop_front() {
                for (i, j) in vec![
                    (pi + 1, pj),
                    (pi.wrapping_sub(1), pj),
                    (pi, pj + 1),
                    (pi, pj.wrapping_sub(1)),
                ] {
                    if i < ni && j < nj && maze[i][j] != '#' && dist[i][j] < 0 {
                        queue.push_back((i, j));
                        dist[i][j] = dist[pi][pj] + 1
                    }
                }
            }
            dist
        };
        let dist_s = bfs(start);
        if dist_s[target.0][target.1] < 0 {
            return -1;
        }
        if nb == 0 {
            return dist_s[target.0][target.1];
        }
        stones = stones.into_iter().filter(|s| dist_s[s.0][s.1] >= 0).collect();
        let ns = stones.len();
        if ns == 0 {
            return -1;
        }
        let mut dist_b = Vec::with_capacity(nb);
        let mut db = vec![vec![-1; nb + 2]; nb];
        for k in 0..nb {
            dist_b.push(bfs(buttons[k]));
            db[k][nb + 1] = dist_b[k][target.0][target.1];
            if db[k][nb + 1] < 0 {
                return -1;
            }
        }
        for k in 0..nb {
            let mut sb = i32::MAX;
            for l in 0..ns {
                let si = stones[l].0;
                let sj = stones[l].1;
                sb = sb.min(dist_s[si][sj] + dist_b[k][si][sj]);
            }
            db[k][nb] = sb;
            for l in k + 1..nb {
                let mut sb = i32::MAX;
                for m in 0..ns {
                    let si = stones[m].0;
                    let sj = stones[m].1;
                    sb = sb.min(dist_b[k][si][sj] + dist_b[l][si][sj]);
                }
                db[k][l] = sb;
                db[l][k] = sb;
            }
        }
        let mut dp = vec![vec![i32::MAX; nb]; 1 << nb];
        for i in 0..nb {
            dp[1 << i][i] = db[i][nb];
        }
        let full_mask = (1 << nb) - 1;
        for mask in 1..full_mask {
            let mut tmask = mask as usize;
            let inv_mask = (!mask) & full_mask;
            while tmask > 0 {
                let i = tmask.trailing_zeros() as usize;
                tmask ^= 1 << i;
                let mut tinv_mask = inv_mask as usize;
                while tinv_mask > 0 {
                    let j = tinv_mask.trailing_zeros() as usize;
                    tinv_mask ^= 1 << j;
                    let nmask = mask | (1 << j);
                    dp[nmask][j] = dp[nmask][j].min(dp[mask][i] + db[i][j]);
                }
            }
        }
        let mut ans = i32::MAX;
        for i in 0..nb {
            ans = ans.min(dp[full_mask][i] + db[i][nb + 1]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimal_steps() {
        assert_eq!(Solution::minimal_steps(vec_str!["S#O", "M..", "M.T"]), 16);
        assert_eq!(Solution::minimal_steps(vec_str!["S#O", "M.#", "M.T"]), -1);
        assert_eq!(Solution::minimal_steps(vec_str!["S#O", "M.T", "M.."]), 17);
    }
}

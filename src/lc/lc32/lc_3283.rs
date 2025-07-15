// https://leetcode.com/problems/maximum-number-of-moves-to-kill-all-pawns/
// 3283. Maximum Number of Moves to Kill All Pawns
pub struct Solution;
impl Solution {
    pub fn max_moves(kx: i32, ky: i32, positions: Vec<Vec<i32>>) -> i32 {
        let mut idx = vec![vec![usize::MAX; 50]; 50];
        let np = positions.len();
        let pawns = [(kx as usize, ky as usize)]
            .into_iter()
            .chain(positions.into_iter().map(|x| (x[0] as usize, x[1] as usize)))
            .collect::<Vec<_>>();
        for (i, &(x, y)) in pawns.iter().enumerate() {
            idx[x][y] = i;
        }
        let mut dist = vec![vec![0; pawns.len()]; pawns.len()];
        let dir = vec![(1, 2), (2, 1), (-1, 2), (-2, 1), (1, -2), (2, -1), (-1, -2), (-2, -1)];
        let mut visited = vec![vec![false; 50]; 50];
        for i in 0..pawns.len() {
            let mut cnt = 0;
            let mut q = std::collections::VecDeque::new();
            q.push_back((pawns[i], 0));
            for v in &mut visited {
                v.fill(false);
            }
            while let Some(((cx, cy), d)) = q.pop_front() {
                if visited[cx][cy] {
                    continue;
                }
                visited[cx][cy] = true;
                if idx[cx][cy] != usize::MAX && idx[cx][cy] >= i {
                    dist[i][idx[cx][cy]] = d;
                    dist[idx[cx][cy]][i] = d;
                    cnt += 1;
                    if cnt == pawns.len() - i {
                        break;
                    }
                }
                for &(dx, dy) in &dir {
                    let nx = (cx as i32 + dx) as usize;
                    let ny = (cy as i32 + dy) as usize;
                    if nx >= 50 || ny >= 50 || visited[nx][ny] {
                        continue;
                    }
                    q.push_back(((nx, ny), d + 1));
                }
            }
        }
        let mut res = vec![vec![0; np]; 1 << np];
        for mask in 1..(1 << np) as usize {
            let n1 = mask.count_ones() as usize;
            if n1 == 1 {
                continue;
            }
            let mut m = mask;
            let want_max = (np ^ n1) & 1 == 1;
            while m > 0 {
                let mut r = if want_max { 0 } else { i32::MAX };
                let i = m.trailing_zeros() as usize;
                m ^= 1 << i;
                let cmask = mask ^ (1 << i);
                let mut cm = cmask;
                while cm > 0 {
                    let j = cm.trailing_zeros() as usize;
                    cm ^= 1 << j;
                    let nd = dist[i + 1][j + 1] + res[cmask][j];
                    if want_max {
                        r = r.max(nd);
                    } else {
                        r = r.min(nd)
                    }
                }
                res[mask][i] = r;
            }
        }
        let mut ans = 0;
        let mask = (1 << np) - 1;
        for i in 0..np {
            ans = ans.max(res[mask][i] + dist[0][i + 1]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_moves() {
        assert_eq!(Solution::max_moves(0, 0, vec_vec![[6, 9], [2, 8], [0, 10]]), 12);
        assert_eq!(Solution::max_moves(1, 1, vec_vec![[0, 0]]), 4);
        assert_eq!(Solution::max_moves(0, 2, vec_vec![[1, 1], [2, 2], [3, 3]]), 8);
        assert_eq!(Solution::max_moves(0, 0, vec_vec![[1, 2], [2, 4]]), 3);
    }
}

// https://leetcode.cn/problems/kskhHQ/
// LCP 71. 集水器
pub struct Solution;
impl Solution {
    pub fn reservoir(shape: Vec<String>) -> i32 {
        let ni = shape.len();
        let nj = shape[0].len();
        let mut block = vec![vec![vec![0; 4]; nj + 1]; ni + 1];
        let mut c = 1;
        for i in 0..ni {
            for j in 0..nj {
                for k in 0..4 {
                    block[i][j][k] = c;
                    c += 1;
                }
            }
        }
        let mut uf = (0..=c).collect::<Vec<_>>();
        fn find(uf: &mut Vec<usize>, x: usize) -> usize {
            if uf[x] != x {
                uf[x] = find(uf, uf[x]);
            }
            uf[x]
        }
        fn union(uf: &mut Vec<usize>, x: usize, y: usize) {
            let x = find(uf, x);
            let y = find(uf, y);
            uf[x] = y;
        }
        const U: usize = 0;
        const D: usize = 1;
        const L: usize = 2;
        const R: usize = 3;
        let mut water = vec![false; c + 1];
        for i in (0..ni).rev() {
            union(&mut uf, block[i][0][L], 0);
            for (j, b) in shape[i].chars().enumerate() {
                union(&mut uf, block[i][j][R], block[i][j + 1][L]);
                union(&mut uf, block[i][j][D], block[i + 1][j][U]);
                if b == 'l' {
                    union(&mut uf, block[i][j][L], block[i][j][D]);
                    union(&mut uf, block[i][j][R], block[i][j][U]);
                } else if b == 'r' {
                    union(&mut uf, block[i][j][R], block[i][j][D]);
                    union(&mut uf, block[i][j][L], block[i][j][U]);
                } else {
                    union(&mut uf, block[i][j][L], block[i][j][D]);
                    union(&mut uf, block[i][j][R], block[i][j][U]);
                    union(&mut uf, block[i][j][L], block[i][j][R])
                }
            }
            for j in 0..nj {
                for k in 0..4 {
                    let idx = block[i][j][k];
                    if find(&mut uf, idx) != find(&mut uf, 0) {
                        water[idx] = true;
                    }
                }
            }
        }
        for j in 0..nj {
            union(&mut uf, block[0][j][U], 0);
        }
        let mut cnt = 0;
        for i in 1..=c {
            if water[i] && find(&mut uf, i) == find(&mut uf, 0) {
                cnt += 1;
            }
        }
        cnt / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::reservoir(vec_str!["....rl", "l.lr.r", ".l..r.", "..lr.."]),
            18
        );
        assert_eq!(
            Solution::reservoir(vec_str![".rlrlrlrl", "ll..rl..r", ".llrrllrr", "..lr..lr."]),
            18
        );
        assert_eq!(Solution::reservoir(vec_str!["rlrr", "llrl", "llr."]), 6);
        assert_eq!(
            Solution::reservoir(vec_str![
                "...rl...", "..r..l..", ".r.rl.l.", "r.r..l.l", "l.l..rl.", ".l.lr.r.", "..l..r..", "...lr..."
            ]),
            30
        );
    }
}

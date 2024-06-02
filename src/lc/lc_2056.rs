// https://leetcode.com/problems/number-of-valid-move-combinations-on-chessboard/
// 2056. Number of Valid Move Combinations On Chessboard
pub struct Solution;
impl Solution {
    pub fn count_combinations(pieces: Vec<String>, positions: Vec<Vec<i32>>) -> i32 {
        let mut dirs = vec![];
        for p in pieces {
            if p == "rook" {
                dirs.push(vec![(1, 0), (0, 1), (-1, 0), (0, -1)]);
            } else if p == "queen" {
                dirs.push(vec![
                    (1, 0),
                    (0, 1),
                    (-1, 0),
                    (0, -1),
                    (1, 1),
                    (1, -1),
                    (-1, 1),
                    (-1, -1),
                ]);
            } else if p == "bishop" {
                dirs.push(vec![(1, 1), (1, -1), (-1, 1), (-1, -1)]);
            } else {
                unreachable!();
            }
        }
        let mut count = 1;
        let mut cur = vec![0; dirs.len()];
        let mut set1 = std::collections::HashSet::new();
        loop {
            let mut q = vec![(positions.clone(), (1 << positions.len()) - 1)];
            let mut first = true;
            while let Some((cpos, mask)) = q.pop() {
                let mut m = mask;
                while m > 0 {
                    let mut pos = cpos.clone();
                    let mut legal = true;
                    for i in 0..pos.len() {
                        if (m >> i) & 1 == 0 {
                            continue;
                        }
                        pos[i][0] = pos[i][0] + dirs[i][cur[i]].0;
                        pos[i][1] = pos[i][1] + dirs[i][cur[i]].1;
                        if pos[i][0] == 0 || pos[i][0] > 8 || pos[i][1] == 0 || pos[i][1] > 8 {
                            legal = false;
                            break;
                        }
                    }
                    if legal {
                        let set = pos.iter().cloned().collect::<std::collections::HashSet<_>>();
                        if set.len() != pos.len() {
                            legal = false;
                        }
                    }
                    if legal {
                        if first {
                            if set1.contains(&pos) {
                                legal = false;
                            } else {
                                set1.insert(pos.clone());
                            }
                        }
                    }
                    if legal {
                        count += 1;
                        q.push((pos, m));
                    }
                    m = (m - 1) & mask;
                }
                first = false;
            }
            let mut add = 1;
            for i in 0..cur.len() {
                cur[i] += add;
                if cur[i] >= dirs[i].len() {
                    cur[i] -= dirs[i].len();
                } else {
                    add = 0;
                    break;
                }
            }
            if add == 1 {
                break;
            }
        }
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_combinations() {
        assert_eq!(
            Solution::count_combinations(vec_str!["rook", "rook"], vec_vec![[1, 1], [8, 8]]),
            223
        );
        assert_eq!(Solution::count_combinations(vec_str!["rook"], vec_vec![[1, 1]]), 15);
        assert_eq!(Solution::count_combinations(vec_str!["queen"], vec_vec![[1, 1]]), 22);
        assert_eq!(Solution::count_combinations(vec_str!["bishop"], vec_vec![[4, 3]]), 12);
    }
}

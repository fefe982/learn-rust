// https://leetcode.com/problems/maximum-value-sum-by-placing-three-rooks-ii/
// 3257. Maximum Value Sum by Placing Three Rooks II
pub struct Solution;
impl Solution {
    pub fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
        let ni = board.len();
        let nj = board[0].len();
        let mut board = board
            .into_iter()
            .enumerate()
            .map(|(i, v)| v.into_iter().enumerate().map(move |(j, n)| (n, i, j)))
            .flatten()
            .collect::<Vec<_>>();
        board.sort_unstable_by_key(|v| -v.0);
        let mut cnti = vec![0; ni];
        let mut cntj = vec![0; nj];
        let mut nb = vec![];
        for idx in 0..(ni + nj) * 2 - 3 {
            let (n, i, j) = board[idx];
            cnti[i] += 1;
            cntj[j] += 1;
            if cnti[i] <= 3 && cntj[j] <= 3 {
                nb.push((n as i64, i, j));
            }
        }
        let mut max = i64::MIN;
        for i in 0..nb.len() - 2 {
            if nb[i].0 + nb[i + 1].0 + nb[i + 2].0 <= max {
                break;
            }
            let mut lmax = i64::MIN;
            for j in i + 1..nb.len() - 1 {
                if nb[i].1 == nb[j].1 || nb[i].2 == nb[j].2 {
                    continue;
                }
                if nb[j].0 + nb[j + 1].0 <= lmax {
                    break;
                }
                for k in j + 1..nb.len() {
                    if nb[i].1 != nb[k].1 && nb[i].2 != nb[k].2 && nb[j].1 != nb[k].1 && nb[j].2 != nb[k].2 {
                        lmax = lmax.max(nb[j].0 + nb[k].0);
                        break;
                    }
                }
            }
            if lmax != i64::MIN {
                max = max.max(nb[i].0 + lmax);
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_value_sum() {
        assert_eq!(
            Solution::maximum_value_sum(vec_vec![[-3, 1, 1, 1], [-3, 1, -3, 1], [-3, 2, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::maximum_value_sum(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            15
        );
        assert_eq!(
            Solution::maximum_value_sum(vec_vec![[1, 1, 1], [1, 1, 1], [1, 1, 1]]),
            3
        );
    }
}

// https://leetcode.com/problems/the-earliest-and-latest-rounds-where-players-compete/
// 1900. The Earliest and Latest Rounds Where Players Compete
pub struct Solution;
impl Solution {
    fn search_with_mem(n: i32, first_p: i32, second_p: i32, mem: &mut Vec<Vec<Vec<Vec<i32>>>>) -> Vec<i32> {
        let first_player = first_p.min(second_p);
        let second_player = first_p.max(second_p);
        if first_player + second_player == n + 1 {
            return vec![1, 1];
        }
        if !mem[n as usize][first_player as usize][second_player as usize].is_empty() {
            return mem[n as usize][first_player as usize][second_player as usize].clone();
        }
        let mut min = n;
        let mut max = 0;
        let first = first_player.min(n + 1 - first_player);
        let second = second_player.min(n + 1 - second_player);
        let (f, s, fleft, rleft) = if first < second {
            (first, second, first == first_player, second == second_player)
        } else {
            (second, first, second == second_player, first == first_player)
        };
        let nn = n / 2 + n % 2;
        for i in 0..f {
            for j in f..s {
                let ilose = i;
                let nf = if fleft { f - ilose } else { nn - ilose };
                let jlose = ilose + j - f + if fleft { 0 } else { 1 };
                let ns = if rleft { s - jlose } else { nn - jlose };
                println!("{i}, {j}, {ilose}, {jlose}, {fleft}, {rleft}, {nf}, {ns}");
                let nminmax = Self::earliest_and_latest(nn, nf, ns);
                min = min.min(nminmax[0] + 1);
                max = max.max(nminmax[1] + 1);
            }
        }
        mem[n as usize][first_player as usize][second_player as usize] = vec![min, max];
        vec![min, max]
    }
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        Self::search_with_mem(
            n,
            first_player,
            second_player,
            &mut vec![vec![vec![vec![]; n as usize + 1]; n as usize + 1]; n as usize + 1],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_earliest_and_latest() {
        assert_eq!(Solution::earliest_and_latest(11, 2, 4), [3, 4]);
        assert_eq!(Solution::earliest_and_latest(5, 1, 5), [1, 1]);
    }
}

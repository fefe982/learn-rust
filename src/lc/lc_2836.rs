// https://leetcode.com/problems/maximize-value-of-function-in-a-ball-passing-game/
// 2836. Maximize Value of Function In a Ball Passing Game
pub struct Solution;
impl Solution {
    pub fn get_max_function_value(receiver: Vec<i32>, k: i64) -> i64 {
        let mut full = (0..receiver.len() as i64).collect::<Vec<_>>();
        let mut end = (0..receiver.len()).collect::<Vec<_>>();
        let mut path = receiver.iter().map(|x| *x as i64).collect::<Vec<_>>();
        let mut receiver = receiver.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut bit = 1;
        if k & 1 != 0 {
            for i in 0..receiver.len() {
                full[i] += path[i];
                end[i] = receiver[end[i]];
            }
        }
        while bit < k {
            bit <<= 1;
            let mut nrec = vec![0; receiver.len()];
            let mut npath = path.clone();
            for i in 0..receiver.len() {
                npath[i] += path[receiver[i]];
                nrec[i] = receiver[receiver[i]];
            }
            if bit & k != 0 {
                for i in 0..receiver.len() {
                    full[i] += npath[end[i]];
                    end[i] = nrec[end[i]];
                }
            }
            path = npath;
            receiver = nrec;
        }
        *full.iter().max().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_max_function_value() {
        assert_eq!(Solution::get_max_function_value(vec![2, 0, 1], 4), 6);
        assert_eq!(Solution::get_max_function_value(vec![1, 1, 1, 2, 3], 3), 10);
    }
}

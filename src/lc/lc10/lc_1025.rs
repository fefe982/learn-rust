// https://leetcode.com/problems/divisor-game/
// 1025. Divisor Game
pub struct Solution;
impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        let n = n as usize;
        let mut v = vec![0; n + 1];
        v[1] = 1;
        fn dfs(n: usize, v: &mut Vec<usize>) -> bool {
            if v[n] != 0 {
                return v[n] == 2;
            }
            for i in 1..n {
                if n % i == 0 {
                    if !dfs(n - i, v) {
                        v[n] = 2;
                        return true;
                    }
                    if i != 1 && n / i != i {
                        if !dfs(n - n / i, v) {
                            v[n] = 2;
                            return true;
                        }
                    }
                }
                if i * i >= n {
                    break;
                }
            }
            v[n] = 1;
            false
        }
        dfs(n, &mut v)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn divisor_game() {
        assert_eq!(Solution::divisor_game(2), true);
        assert_eq!(Solution::divisor_game(3), false);
    }
}

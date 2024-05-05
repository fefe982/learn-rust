// https://leetcode.com/problems/defuse-the-bomb/
// 1652. Defuse the Bomb
pub struct Solution;
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![0; code.len()];
        }
        let (mut s, mut e) = if k > 0 {
            (1, k as usize)
        } else {
            ((code.len() as i32 + k) as usize, code.len() - 1)
        };
        let mut sum = 0;
        for i in s..=e {
            sum += code[i];
        }
        let mut ans = vec![];
        for _ in 0..code.len() {
            ans.push(sum);
            sum -= code[s];
            s = (s + 1) % code.len();
            e = (e + 1) % code.len();
            sum += code[e];
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decrypt() {
        assert_eq!(Solution::decrypt(vec![5, 7, 1, 4], 3), [12, 10, 16, 13]);
        assert_eq!(Solution::decrypt(vec![1, 2, 3, 4], 0), [0, 0, 0, 0]);
        assert_eq!(Solution::decrypt(vec![2, 4, 9, 3], -2), [12, 5, 6, 13]);
    }
}

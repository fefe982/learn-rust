// https://leetcode.com/problems/perfect-squares/
// 279. Perfect Squares
pub struct Solution;
impl Solution {
    fn dfs(n: i32, mem: &mut Vec<i32>) -> i32 {
        if mem[n as usize] > 0 {
            return mem[n as usize];
        }
        let pos = (n as f64).sqrt() as i32;
        if pos * pos == n {
            mem[n as usize] = 1;
            return 1;
        }
        let mut mincnt = n;
        for p in (1..=pos).rev() {
            let limit = p * p;
            if limit * mincnt < n {
                break;
            }
            let c = Self::dfs(n - limit, mem);
            if c + 1 < mincnt {
                mincnt = c + 1;
            }
        }
        mem[n as usize] = mincnt;
        return mincnt;
    }
    pub fn num_squares(n: i32) -> i32 {
        Self::dfs(n, &mut vec![0; (n + 1) as usize])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_squares() {
        assert_eq!(Solution::num_squares(192), 3);
        assert_eq!(Solution::num_squares(12), 3);
        assert_eq!(Solution::num_squares(13), 2);
    }
}

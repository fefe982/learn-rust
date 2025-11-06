// https://leetcode.com/problems/xor-after-range-multiplication-queries-ii/
// 3655. XOR After Range Multiplication Queries II
pub struct Solution;
impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const M: i64 = 1_000_000_007;
        let mut nums = nums;
        let ns = (queries.len() as f64).sqrt() as usize;
        let mut save = vec![vec![]; ns];
        for q in queries {
            let (l, r, s, f) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as i64);
            let k = (r - l) / s + 1;
            if s < ns && k != 1 {
                save[s].push((l, k, f));
            } else {
                for i in 0..k {
                    nums[l + i * s] = (nums[l + i * s] as i64 * f % M) as i32;
                }
            }
        }
        let inv = |n: i64| -> i64 {
            let mut a = 1;
            let mut b = n;
            while a % b != 0 {
                let t = M / b + 1;
                b = b * t % M;
                a = a * t % M;
            }
            a / b
        };
        for (idx, save_q) in save.into_iter().enumerate() {
            if save_q.is_empty() {
                continue;
            }
            let mut g = vec![vec![]; idx];
            for (l, k, f) in save_q {
                g[l % idx].push((l, k, f));
            }
            for (ir, rg) in g.into_iter().enumerate() {
                if rg.is_empty() {
                    continue;
                }
                if rg.len() == 1 {
                    let (l, k, f) = rg[0];
                    for i in 0..k {
                        nums[l + i * idx] = (nums[l + i * idx] as i64 * f % M) as i32;
                    }
                    continue;
                }
                let mut mul = vec![1; (nums.len() - 1) / idx + 2];
                for (l, k, f) in rg {
                    let il = l / idx;
                    mul[il] = mul[il] * f % M;
                    mul[il + k] = mul[il + k] * inv(f) % M;
                }
                let mut f = 1;
                for i in 0..(nums.len() - ir + idx - 1) / idx {
                    f = f * mul[i] % M;
                    nums[i * idx + ir] = (nums[i * idx + ir] as i64 * f % M) as i32;
                }
            }
        }
        nums.into_iter().fold(0, |ans, x| ans ^ x)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn xor_after_queries() {
        assert_eq!(
            Solution::xor_after_queries(
                vec![984, 236],
                vec_vec![
                    [0, 0, 1, 5],
                    [0, 1, 2, 16],
                    [1, 1, 2, 11],
                    [0, 1, 1, 10],
                    [0, 1, 1, 12],
                    [0, 0, 2, 19],
                    [0, 1, 2, 7],
                    [1, 1, 2, 19],
                    [1, 1, 1, 2],
                    [1, 1, 1, 18],
                    [0, 1, 2, 9],
                    [1, 1, 1, 17],
                    [1, 1, 2, 18],
                    [0, 0, 2, 3],
                    [1, 1, 1, 15],
                    [1, 1, 2, 14],
                    [1, 1, 1, 19],
                    [0, 0, 2, 6],
                    [0, 0, 1, 18],
                    [1, 1, 2, 12]
                ]
            ),
            596888069
        );
        assert_eq!(
            Solution::xor_after_queries(
                vec![780],
                vec_vec![
                    [0, 0, 1, 13],
                    [0, 0, 1, 17],
                    [0, 0, 1, 9],
                    [0, 0, 1, 18],
                    [0, 0, 1, 16],
                    [0, 0, 1, 6],
                    [0, 0, 1, 4],
                    [0, 0, 1, 11],
                    [0, 0, 1, 7],
                    [0, 0, 1, 18],
                    [0, 0, 1, 8],
                    [0, 0, 1, 15],
                    [0, 0, 1, 12]
                ]
            ),
            523618060
        );
        assert_eq!(Solution::xor_after_queries(vec![1, 1, 1], vec_vec![[0, 2, 1, 4]]), 4);
        assert_eq!(
            Solution::xor_after_queries(vec![2, 3, 1, 5, 4], vec_vec![[1, 4, 2, 3], [0, 2, 1, 2]]),
            31
        );
    }
}

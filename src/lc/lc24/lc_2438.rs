// https://leetcode.com/problems/range-product-queries-of-powers/
// 2438. Range Product Queries of Powers
pub struct Solution;
impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut powers = vec![];
        let mut p = 1;
        let mut n = n;
        while n > 0 {
            if n & 1 == 1 {
                powers.push(p);
            }
            p *= 2;
            n >>= 1;
        }
        let mut mul = vec![vec![0; powers.len()]; powers.len()];
        for i in 0..powers.len() {
            mul[i][i] = powers[i];
            for j in i + 1..powers.len() {
                mul[i][j] = ((mul[i][j - 1] as i64 * powers[j] as i64) % 1000000007) as i32;
            }
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            ans.push(mul[q[0] as usize][q[1] as usize])
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn product_queries() {
        assert_eq!(
            Solution::product_queries(15, vec_vec![[0, 1], [2, 2], [0, 3]]),
            [2, 4, 64]
        );
        assert_eq!(Solution::product_queries(2, vec_vec![[0, 0]]), [2]);
    }
}

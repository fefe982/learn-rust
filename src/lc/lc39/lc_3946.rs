// https://leetcode.com/problems/maximum-number-of-items-from-sale-i/
// 3946. Maximum Number of Items From the Sale I
pub struct Solution;
impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let b = budget as usize;
        let mut f = vec![0; b + 1];
        let mut min = usize::MAX;
        for item in &items {
            let (factor, price) = (item[0], item[1] as usize);
            min = min.min(price);
            let mut cnt = 0;
            for other in &items {
                if other[0] % factor == 0 {
                    cnt += 1;
                }
            }
            for i in (price..=b).rev() {
                f[i] = f[i].max(f[i - price] + cnt);
            }
        }
        let mut r = 0;
        for j in 0..=b {
            r = r.max(f[j] + (b - j) / min);
        }
        r as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_sale_items() {
        assert_eq!(Solution::maximum_sale_items(vec_vec![[6, 2], [2, 6], [3, 4]], 9), 4);
        assert_eq!(
            Solution::maximum_sale_items(vec_vec![[2, 4], [3, 2], [4, 1], [6, 4], [12, 4]], 8),
            10
        );
    }
}

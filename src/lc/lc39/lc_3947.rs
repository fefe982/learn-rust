// https://leetcode.com/problems/maximum-number-of-items-from-sale-ii/
// 3947. Maximum Number of Items From Sale II
pub struct Solution;
impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let mut items = items;
        let maxfactor = items.iter().map(|v| v[0]).max().unwrap() as usize;
        let mut f = vec![0; maxfactor + 1];
        for item in &items {
            f[item[0] as usize] += 1;
        }
        for i in 1..=maxfactor as usize {
            if f[i] > 0 {
                for j in (2 * i..=maxfactor).step_by(i) {
                    f[i] += f[j];
                }
            }
        }
        items.sort_unstable_by_key(|v| v[1]);
        let min = items[0][1];
        let mut ans = 0;
        let mut budget = budget;
        for item in &items {
            let factor = item[0] as usize;
            let price = item[1];
            if price - min > min {
                break;
            }
            let cnt = f[factor] - 1;
            let take = cnt.min(budget / price);
            ans += take * 2;
            budget -= take * price;
        }
        ans + budget / min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_sale_items() {
        assert_eq!(Solution::maximum_sale_items(vec_vec![[1, 6], [2, 4], [3, 5]], 19), 5);
        assert_eq!(
            Solution::maximum_sale_items(vec_vec![[2, 8], [1, 10], [6, 6], [4, 12], [5, 20], [5, 17]], 35),
            7
        );
    }
}

// https://leetcode.com/problems/shopping-offers/
// 638. Shopping Offers
pub struct Solution;
impl Solution {
    fn sub(mut a: i32, mut b: i32) -> i32 {
        let mut res = 0;
        let mut mul = 1;
        while a > 0 || b > 0 {
            let diff = a % 11 - b % 11;
            if diff < 0 {
                return -1;
            }
            res += diff * mul;
            mul *= 11;
            a /= 11;
            b /= 11;
        }
        res
    }
    fn shopping_offers_inner(
        special: &Vec<(i32, i32)>,
        needs: i32,
        save: &mut std::collections::HashMap<i32, i32>,
    ) -> i32 {
        if needs == 0 {
            return 0;
        }
        if let Some(&ans) = save.get(&needs) {
            return ans;
        }
        let mut ans = 0;
        for &(s, p) in special {
            let diff = Self::sub(needs, s);
            if diff >= 0 {
                ans = ans.max(p + Self::shopping_offers_inner(special, diff, save));
            }
        }
        save.insert(needs, ans);
        ans
    }
    fn convert(price: &Vec<i32>, counts: Vec<i32>) -> (i32, i32) {
        price
            .iter()
            .zip(counts)
            .fold((0, 0), |(s, p), (price, count)| (s * 11 + count, price * count + p))
    }
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let special = special
            .into_iter()
            .filter_map(|mut x| {
                let ps = x.pop().unwrap();
                let (s, p) = Self::convert(&price, x);
                if ps < p {
                    Some((s, p - ps))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let (need_state, need_total) = Self::convert(&price, needs);
        let save = Self::shopping_offers_inner(&special, need_state, &mut std::collections::HashMap::new());
        need_total - save
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn shopping_offers() {
        assert_eq!(
            Solution::shopping_offers(vec![2, 5], vec_vec![[3, 0, 5], [1, 2, 10]], vec![3, 2]),
            14
        );
        assert_eq!(
            Solution::shopping_offers(vec![2, 3, 4], vec_vec![[1, 1, 0, 4], [2, 2, 1, 9]], vec![1, 2, 1]),
            11
        );
    }
}

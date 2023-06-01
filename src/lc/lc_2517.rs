// https://leetcode.com/problems/maximum-tastiness-of-candy-basket/
// 2517. Maximum Tastiness of Candy Basket
pub struct Solution;
impl Solution {
    fn check_gap(price: &Vec<i32>, gap: i32, k: i32) -> bool {
        let mut n = 1;
        let mut s = price[0];
        let l = price.len();
        loop {
            let pos = price.partition_point(|&x| x < s + gap);
            if pos >= l {
                return n >= k;
            }
            n += 1;
            s = price[pos];
            if n >= k {
                return true;
            }
        }
    }
    pub fn maximum_tastiness(mut price: Vec<i32>, k: i32) -> i32 {
        price.sort_unstable();
        let l = price.len();
        let min = price[0];
        let max = price[l - 1];
        if max - min < k - 1 {
            return 0;
        }
        let mut low = i32::MAX;
        for i in 1..l {
            low = std::cmp::min(low, price[i] - price[i - 1]);
        }
        let mut high = (max - min) / (k - 1);
        if Self::check_gap(&price, high, k) {
            return high;
        }
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Self::check_gap(&price, mid, k) {
                low = mid;
            } else {
                high = mid;
            }
        }
        low
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_tastiness() {
        assert_eq!(Solution::maximum_tastiness(vec![13, 5, 1, 8, 21, 2], 3), 8);
        assert_eq!(Solution::maximum_tastiness(vec![1, 3, 1], 2), 2);
        assert_eq!(Solution::maximum_tastiness(vec![7, 7, 7, 7], 2), 0);
    }
}

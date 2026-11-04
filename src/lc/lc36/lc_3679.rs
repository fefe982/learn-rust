// https://leetcode.com/problems/minimum-discards-to-balance-inventory/
// 3679. Minimum Discards to Balance Inventory
pub struct Solution;
impl Solution {
    pub fn min_arrivals_to_discard(arrivals: Vec<i32>, w: i32, m: i32) -> i32 {
        let mut count = std::collections::HashMap::new();
        let mut arrivals = arrivals;
        let mut drop = 0;
        let w = w as usize;
        for i in 0..arrivals.len() {
            let c = count.entry(arrivals[i]).or_insert(0);
            if *c == m {
                drop += 1;
                arrivals[i] = -1;
            } else {
                *c += 1;
            }
            if i + 1 >= w {
                let j = i + 1 - w;
                if arrivals[j] != -1 {
                    *count.entry(arrivals[j]).or_insert(0) -= 1;
                }
            }
        }
        drop
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_arrivals_to_discard() {
        assert_eq!(Solution::min_arrivals_to_discard(vec![1, 2, 1, 3, 1], 4, 2), 0);
        assert_eq!(Solution::min_arrivals_to_discard(vec![1, 2, 3, 3, 3, 4], 3, 2), 1);
    }
}

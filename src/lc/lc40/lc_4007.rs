// https://leetcode.com/problems/widest-possible-fence/
// 4007. Widest Possible Fence
pub struct Solution;
impl Solution {
    pub fn maximum_width(planks: Vec<i32>) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        for &p in &planks {
            *cnt.entry(p).or_insert(0) += 1;
        }
        let mut cntf = cnt.clone();
        for (&l1, &c1) in &cnt {
            *cntf.entry(l1 * 2).or_insert(0) += c1 / 2;
            for (&l2, &c2) in &cnt {
                if l2 > l1 {
                    *cntf.entry(l1 + l2).or_insert(0) += c1.min(c2);
                }
            }
        }
        *cntf.values().max().unwrap_or(&0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_width() {
        assert_eq!(Solution::maximum_width(vec![1, 3, 2, 5, 7, 5, 4, 2, 1]), 4);
        assert_eq!(Solution::maximum_width(vec![2, 3, 7]), 1);
    }
}

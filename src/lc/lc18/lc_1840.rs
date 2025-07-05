// https://leetcode.com/problems/maximum-building-height/
// 1840. Maximum Building Height
pub struct Solution;
impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        if restrictions.len() == 0 {
            return n - 1;
        }
        let mut restrictions = restrictions;
        restrictions.sort_by_key(|x| x[0]);
        if restrictions.last().unwrap()[0] != n {
            restrictions.push(vec![n, n - 1]);
        }
        let mut max = 0;
        let mut h = restrictions.last().unwrap()[1];
        let mut ih = restrictions.last().unwrap()[0];
        for r in restrictions.iter_mut().rev().skip(1) {
            r[1] = (h + ih - r[0]).min(r[1]);
            h = r[1];
            ih = r[0];
        }
        h = 0;
        ih = 1;
        for r in restrictions.iter() {
            let nh = (h + r[0] - ih).min(r[1]);
            let m = (h + r[0] - ih) - ((h + r[0] - ih) - nh + 1) / 2;
            max = max.max(m);
            h = nh;
            ih = r[0];
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_building() {
        assert_eq!(Solution::max_building(5, vec_vec![[2, 1], [4, 1]]), 2);
        assert_eq!(Solution::max_building(6, Vec::new()), 5);
        assert_eq!(Solution::max_building(10, vec_vec![[5, 3], [2, 5], [7, 4], [10, 3]]), 5);
    }
}

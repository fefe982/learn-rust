// https://leetcode.com/problems/delete-and-earn/
// 740. Delete and Earn
pub struct Solution;
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut m = std::collections::BTreeMap::new();
        for n in nums {
            *m.entry(n).or_insert(0) += 1;
        }
        let mut take = 0;
        let mut skip = 0;
        let mut last = -1;
        for (k, v) in m {
            if k == last + 1 {
                let ntake = skip + k * v;
                let nskip = take.max(skip);
                take = ntake;
                skip = nskip;
            } else {
                skip = take.max(skip);
                take = skip + k * v;
            }
            last = k;
        }
        take.max(skip)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn delete_and_earn() {
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }
}

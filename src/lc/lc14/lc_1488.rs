// https://leetcode.com/problems/avoid-flood-in-the-city/
// 1488. Avoid Flood in The City
pub struct Solution;
impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut res = vec![-1; rains.len()];
        let mut full = std::collections::HashMap::<i32, usize>::new();
        let mut avail = std::collections::BTreeSet::<usize>::new();
        for (i, &r) in rains.iter().enumerate() {
            if r > 0 {
                if let Some(j) = full.insert(r, i) {
                    if let Some(&drain) = avail.range(j..).next() {
                        avail.remove(&drain);
                        res[drain] = r;
                    } else {
                        return vec![];
                    }
                }
            } else {
                avail.insert(i);
            }
        }
        for i in avail {
            res[i] = 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_avoid_flood() {
        assert_eq!(
            Solution::avoid_flood(vec![69, 0, 0, 0, 69]),
            vec![-1, 69, 1, 1, -1]
        );
        assert_eq!(
            Solution::avoid_flood(vec![1, 2, 0, 0, 2, 1]),
            vec![-1, -1, 2, 1, -1, -1]
        );
        assert_eq!(Solution::avoid_flood(vec![1, 2, 0, 1, 2]), vec![]);
    }
}

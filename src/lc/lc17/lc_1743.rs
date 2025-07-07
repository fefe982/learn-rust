// https://leetcode.com/problems/restore-the-array-from-adjacent-pairs/
// 1743. Restore the Array From Adjacent Pairs
pub struct Solution;
impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = std::collections::BTreeMap::<i32, std::collections::HashSet<i32>>::new();
        for pair in adjacent_pairs {
            m.entry(pair[0]).or_default().insert(pair[1]);
            m.entry(pair[1]).or_default().insert(pair[0]);
        }
        let mut res = vec![];
        for (&k, v) in m.iter() {
            if v.len() == 1 {
                res.push(k);
                break;
            }
        }
        loop {
            let back = res.last().unwrap();
            if let Some(&next) = m.remove(back).unwrap().iter().next() {
                m.get_mut(&next).unwrap().remove(&back);
                res.push(next);
            } else {
                break;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_restore_array() {
        assert_eq!(
            Solution::restore_array(vec_vec![[2, 1], [3, 4], [3, 2]]),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            Solution::restore_array(vec_vec![[4, -2], [1, 4], [-3, 1]]),
            vec![-3, 1, 4, -2]
        );
        assert_eq!(
            Solution::restore_array(vec_vec![[100000, -100000]]),
            vec![-100000, 100000]
        );
    }
}

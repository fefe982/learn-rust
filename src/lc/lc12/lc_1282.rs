// https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/
// 1282. Group the People Given the Group Size They Belong To
pub struct Solution;
impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut groups = Vec::new();
        let mut group = std::collections::HashMap::<i32, Vec<i32>>::new();
        for (i, &size) in group_sizes.iter().enumerate() {
            group.entry(size).or_default().push(i as i32);
            if group.get(&size).unwrap().len() == size as usize {
                groups.push(group.remove(&size).unwrap());
            }
        }
        groups
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn group_the_people() {
        assert_eq!(
            Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
            vec_vec![[0, 1, 2], [5], [3, 4, 6]]
        );
        assert_eq!(
            Solution::group_the_people(vec![2, 1, 3, 3, 3, 2]),
            vec_vec![[1], [2, 3, 4], [0, 5]]
        );
    }
}

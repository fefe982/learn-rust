// https://leetcode.com/problems/minimum-index-sum-of-two-lists/
// 599. Minimum Index Sum of Two Lists
pub struct Solution;
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut map = std::collections::HashMap::new();
        for (i, v) in list1.iter().enumerate() {
            map.insert(v, i);
        }
        let mut min = list1.len() + list2.len();
        let mut ret = vec![];
        for (i, v) in list2.iter().enumerate() {
            if let Some(&j) = map.get(v) {
                let sum = i + j;
                if sum < min {
                    min = sum;
                    ret = vec![v.clone()];
                } else if sum == min {
                    ret.push(v.clone());
                }
            }
        }
        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_restaurant() {
        assert_sort_eq!(
            Solution::find_restaurant(
                vec_str!["Shogun", "Tapioca Express", "Burger King", "KFC"],
                vec_str![
                    "Piatti",
                    "The Grill at Torrey Pines",
                    "Hungry Hunter Steakhouse",
                    "Shogun"
                ]
            ),
            vec_str!["Shogun"]
        );
        assert_sort_eq!(
            Solution::find_restaurant(
                vec_str!["Shogun", "Tapioca Express", "Burger King", "KFC"],
                vec_str!["KFC", "Shogun", "Burger King"]
            ),
            vec_str!["Shogun"]
        );
        assert_sort_eq!(
            Solution::find_restaurant(vec_str!["happy", "sad", "good"], vec_str!["sad", "happy", "good"]),
            vec_str!["sad", "happy"]
        );
    }
}

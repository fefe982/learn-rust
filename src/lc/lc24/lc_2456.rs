// https://leetcode.com/problems/most-popular-video-creator/
// 2456. Most Popular Video Creator
pub struct Solution;
impl Solution {
    pub fn most_popular_creator(creators: Vec<String>, ids: Vec<String>, views: Vec<i32>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();
        let mut max_view = 0;
        for ((creator, id), &view) in creators.iter().zip(ids.iter()).zip(views.iter()) {
            let entry = map.entry(creator.clone()).or_insert((0, String::new(), 0));
            entry.0 += view as i64;
            if view > entry.2 || (view == entry.2 && (id < &entry.1 || entry.1.is_empty())) {
                entry.1 = id.clone();
                entry.2 = view;
            }
            max_view = max_view.max(entry.0);
        }
        map.into_iter()
            .filter(|(_, (total_views, _, _))| *total_views == max_view)
            .map(|(creator, (_, id, _))| vec![creator, id])
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn most_popular_creator() {
        assert_sort_eq!(
            Solution::most_popular_creator(
                vec_str!["alice", "bob", "alice", "chris"],
                vec_str!["one", "two", "three", "four"],
                vec![5, 10, 5, 4]
            ),
            vec_vec_str![["alice", "one"], ["bob", "two"]]
        );
        assert_sort_eq!(
            Solution::most_popular_creator(
                vec_str!["alice", "alice", "alice"],
                vec_str!["a", "b", "c"],
                vec![1, 2, 2]
            ),
            vec_vec_str![["alice", "b"]]
        );
    }
}

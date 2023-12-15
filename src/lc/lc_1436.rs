// https://leetcode.com/problems/destination-city/
// 1436. Destination City
pub struct Solution;
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut dst = paths[0][1].clone();
        let mut graph = std::collections::HashMap::<String, String>::new();
        for path in paths.into_iter().skip(1) {
            graph.insert(path[0].clone(), path[1].clone());
            while let Some(d) = graph.get(&dst) {
                dst = d.clone();
            }
        }
        dst
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_dest_city() {
        assert_eq!(
            Solution::dest_city(vec_vec_str![
                ["London", "New York"],
                ["New York", "Lima"],
                ["Lima", "Sao Paulo"]
            ]),
            "Sao Paulo"
        );
        assert_eq!(
            Solution::dest_city(vec_vec_str![["B", "C"], ["D", "B"], ["C", "A"]]),
            "A"
        );
        assert_eq!(Solution::dest_city(vec_vec_str![["A", "Z"]]), "Z");
    }
}

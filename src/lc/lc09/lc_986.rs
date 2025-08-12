// https://leetcode.com/problems/interval-list-intersections/
// 986. Interval List Intersections
pub struct Solution;
impl Solution {
    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut j = 0;
        let mut ret = Vec::new();
        while i < first_list.len() && j < second_list.len() {
            let s = first_list[i][0].max(second_list[j][0]);
            let e = first_list[i][1].min(second_list[j][1]);
            if s <= e {
                ret.push(vec![s, e]);
            }
            if first_list[i][1] < second_list[j][1] {
                i += 1;
            } else {
                j += 1;
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
    fn interval_intersection() {
        assert_eq!(
            Solution::interval_intersection(
                vec_vec![[0, 2], [5, 10], [13, 23], [24, 25]],
                vec_vec![[1, 5], [8, 12], [15, 24], [25, 26]]
            ),
            vec_vec![[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]]
        );
        assert_eq!(
            Solution::interval_intersection(vec_vec![[1, 3], [5, 9]], Vec::<Vec<i32>>::new()),
            Vec::<Vec<i32>>::new()
        );
    }
}

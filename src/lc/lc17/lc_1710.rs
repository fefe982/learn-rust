// https://leetcode.com/problems/maximum-units-on-a-truck/
// 1710. Maximum Units on a Truck
pub struct Solution;
impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types;
        box_types.sort_by(|a, b| b[1].cmp(&a[1]));
        let mut truck_size = truck_size;
        let mut total = 0;
        let mut i = 0;
        while truck_size > 0 && i < box_types.len() {
            let s = truck_size.min(box_types[i][0]);
            total += s * box_types[i][1];
            truck_size -= s;
            i += 1;
        }
        total
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_units() {
        assert_eq!(
            Solution::maximum_units(
                vec_vec![
                    [1, 3],
                    [5, 5],
                    [2, 5],
                    [4, 2],
                    [4, 1],
                    [3, 1],
                    [2, 2],
                    [1, 3],
                    [2, 5],
                    [3, 2]
                ],
                35
            ),
            76
        );
        assert_eq!(Solution::maximum_units(vec_vec![[1, 3], [2, 2], [3, 1]], 4), 8);
        assert_eq!(
            Solution::maximum_units(vec_vec![[5, 10], [2, 5], [4, 7], [3, 9]], 10),
            91
        );
    }
}

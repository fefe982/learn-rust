// https://leetcode.com/problems/filter-restaurants-by-vegan-friendly-price-and-distance/
// 1333. Filter Restaurants by Vegan-Friendly, Price and Distance
pub struct Solution;
impl Solution {
    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let mut v = restaurants
            .into_iter()
            .filter_map(|v| {
                if (v[2] != 0 || vegan_friendly != 1) && v[3] <= max_price && v[4] <= max_distance {
                    Some((std::cmp::Reverse(v[1]), std::cmp::Reverse(v[0])))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v.into_iter().map(|(_, id)| id.0).collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_filter_restaurants() {
        assert_eq!(
            Solution::filter_restaurants(
                vec_vec![
                    [1, 4, 1, 40, 10],
                    [2, 8, 0, 50, 5],
                    [3, 8, 1, 30, 4],
                    [4, 10, 0, 10, 3],
                    [5, 1, 1, 15, 1]
                ],
                1,
                50,
                10
            ),
            vec![3, 1, 5]
        );
        assert_eq!(
            Solution::filter_restaurants(
                vec_vec![
                    [1, 4, 1, 40, 10],
                    [2, 8, 0, 50, 5],
                    [3, 8, 1, 30, 4],
                    [4, 10, 0, 10, 3],
                    [5, 1, 1, 15, 1]
                ],
                0,
                50,
                10
            ),
            vec![4, 3, 2, 1, 5]
        );
        assert_eq!(
            Solution::filter_restaurants(
                vec_vec![
                    [1, 4, 1, 40, 10],
                    [2, 8, 0, 50, 5],
                    [3, 8, 1, 30, 4],
                    [4, 10, 0, 10, 3],
                    [5, 1, 1, 15, 1]
                ],
                0,
                50,
                3
            ),
            vec![4, 5]
        );
    }
}

// https://leetcode.com/problems/display-table-of-food-orders-in-a-restaurant/
// 1418. Display Table of Food Orders in a Restaurant
pub struct Solution;
impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut table = std::collections::BTreeMap::new();
        let mut foods = std::collections::BTreeSet::new();
        for order in orders {
            let table_num = order[1].parse::<i32>().unwrap();
            let food = &order[2];
            foods.insert(food.to_string());
            table
                .entry(table_num)
                .or_insert_with(std::collections::HashMap::new)
                .entry(food.to_string())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        let mut res = vec![vec!["Table".to_string()]];
        for food in &foods {
            res[0].push(food.clone());
        }
        for (table_num, food_count) in table {
            let mut row = vec![table_num.to_string()];
            for food in &foods {
                row.push(food_count.get(food).unwrap_or(&0).to_string());
            }
            res.push(row);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn display_table() {
        assert_eq!(
            Solution::display_table(vec_vec_str![
                ["David", "3", "Ceviche"],
                ["Corina", "10", "Beef Burrito"],
                ["David", "3", "Fried Chicken"],
                ["Carla", "5", "Water"],
                ["Carla", "5", "Ceviche"],
                ["Rous", "3", "Ceviche"]
            ]),
            vec_vec_str![
                ["Table", "Beef Burrito", "Ceviche", "Fried Chicken", "Water"],
                ["3", "0", "2", "1", "0"],
                ["5", "0", "1", "0", "1"],
                ["10", "1", "0", "0", "0"]
            ]
        );
        assert_eq!(
            Solution::display_table(vec_vec_str![
                ["James", "12", "Fried Chicken"],
                ["Rates", "12", "Fried Chicken"],
                ["Amadeus", "12", "Fried Chicken"],
                ["Adam", "1", "Canadian Waffles"],
                ["Brianna", "1", "Canadian Waffles"]
            ]),
            vec_vec_str![
                ["Table", "Canadian Waffles", "Fried Chicken"],
                ["1", "2", "0"],
                ["12", "0", "3"]
            ]
        );
        assert_eq!(
            Solution::display_table(vec_vec_str![
                ["Laura", "2", "Bean Burrito"],
                ["Jhon", "2", "Beef Burrito"],
                ["Melissa", "2", "Soda"]
            ]),
            vec_vec_str![["Table", "Bean Burrito", "Beef Burrito", "Soda"], ["2", "1", "1", "1"]]
        );
    }
}

// https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/
// 2115. Find All Possible Recipes from Given Supplies
pub struct Solution;
impl Solution {
    fn search<'r>(
        r: &'r str,
        r_map: &mut std::collections::HashMap<&'r str, &'r Vec<String>>,
        supplies: &mut std::collections::HashSet<&'r str>,
    ) {
        if let Some(ingredients) = r_map.remove(r) {
            for ingredient in ingredients {
                let ingred = ingredient.as_str();
                if supplies.contains(ingred) {
                    continue;
                }
                Self::search(ingred, r_map, supplies);
                if !supplies.contains(ingred) {
                    return;
                }
            }
            supplies.insert(r);
        }
    }
    pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
        let mut supplies = supplies
            .iter()
            .map(|s| s.as_str())
            .collect::<std::collections::HashSet<_>>();
        let mut r_map = recipes
            .iter()
            .map(|r| r.as_str())
            .zip(ingredients.iter())
            .collect::<std::collections::HashMap<_, _>>();
        let mut ans = vec![];
        for dish in &recipes {
            Self::search(dish.as_str(), &mut r_map, &mut supplies);
            if supplies.contains(dish.as_str()) {
                ans.push(dish.clone());
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_all_recipes() {
        assert_sort_eq!(
            Solution::find_all_recipes(
                vec_str!["ju", "fzjnm", "x", "e", "zpmcz", "h", "q"],
                vec_vec_str![
                    ["d"],
                    ["hveml", "f", "cpivl"],
                    ["cpivl", "zpmcz", "h", "e", "fzjnm", "ju"],
                    ["cpivl", "hveml", "zpmcz", "ju", "h"],
                    ["h", "fzjnm", "e", "q", "x"],
                    ["d", "hveml", "cpivl", "q", "zpmcz", "ju", "e", "x"],
                    ["f", "hveml", "cpivl"]
                ],
                vec_str!["f", "hveml", "cpivl", "d"]
            ),
            ["fzjnm", "ju", "q"]
        );
        assert_sort_eq!(
            Solution::find_all_recipes(
                vec_str!["bread"],
                vec_vec_str![["yeast", "flour"]],
                vec_str!["yeast", "flour", "corn"]
            ),
            ["bread"]
        );
        assert_sort_eq!(
            Solution::find_all_recipes(
                vec_str!["bread", "sandwich"],
                vec_vec_str![["yeast", "flour"], ["bread", "meat"]],
                vec_str!["yeast", "flour", "meat"]
            ),
            ["bread", "sandwich"]
        );
        assert_sort_eq!(
            Solution::find_all_recipes(
                vec_str!["bread", "sandwich", "burger"],
                vec_vec_str![["yeast", "flour"], ["bread", "meat"], ["sandwich", "meat", "bread"]],
                vec_str!["yeast", "flour", "meat"]
            ),
            ["bread", "sandwich", "burger"]
        );
    }
}

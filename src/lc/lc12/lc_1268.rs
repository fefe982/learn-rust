// https://leetcode.com/problems/search-suggestions-system/
// 1268. Search Suggestions System
pub struct Solution;
impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut res = vec![];
        let mut products = products;
        products.sort();
        let mut j = 0;
        let mut k = products.len();
        let search_word = search_word.as_bytes();
        for i in 0..search_word.len() {
            while j < k && (products[j].len() <= i || products[j].as_bytes()[i] < search_word[i]) {
                j += 1;
            }
            while k > j && products[k - 1].len() > i && products[k - 1].as_bytes()[i] > search_word[i] {
                k -= 1;
            }
            res.push(
                products[j..k.min(j + 3)]
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(),
            );
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn suggested_products() {
        assert_eq!(
            Solution::suggested_products(
                vec_str!["mobile", "mouse", "moneypot", "monitor", "mousepad"],
                "mouse".to_string()
            ),
            vec_vec_str![
                ["mobile", "moneypot", "monitor"],
                ["mobile", "moneypot", "monitor"],
                ["mouse", "mousepad"],
                ["mouse", "mousepad"],
                ["mouse", "mousepad"]
            ]
        );
        assert_eq!(
            Solution::suggested_products(vec_str!["havana"], "havana".to_string()),
            [["havana"], ["havana"], ["havana"], ["havana"], ["havana"], ["havana"]]
        );
    }
}

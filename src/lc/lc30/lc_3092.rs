// https://leetcode.com/problems/most-frequent-ids/
// 3092. Most Frequent Even Element
pub struct Solution;
impl Solution {
    pub fn most_frequent_i_ds(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64> {
        let mut num_map = std::collections::HashMap::new();
        let mut freq_map = std::collections::BTreeMap::new();
        let mut ans = Vec::with_capacity(nums.len());
        for (n, f) in nums.into_iter().zip(freq.into_iter()) {
            if f != 0 {
                let ff = num_map.entry(n).or_insert(0);
                if *ff != 0 {
                    let c = freq_map.entry(*ff).or_insert(0);
                    *c -= 1;
                    if *c == 0 {
                        freq_map.remove(ff);
                    }
                }
                *ff += f as i64;
                if *ff != 0 {
                    let c = freq_map.entry(*ff).or_insert(0);
                    *c += 1;
                }
            }
            ans.push(*freq_map.keys().rev().next().unwrap_or(&0))
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn most_frequent_i_ds() {
        assert_eq!(
            Solution::most_frequent_i_ds(vec![2, 3, 2, 1], vec![3, 2, -3, 1]),
            vec![3, 3, 2, 2]
        );
        assert_eq!(Solution::most_frequent_i_ds(vec![5, 5, 3], vec![2, -2, 1]), [2, 0, 1]);
    }
}

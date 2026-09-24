// https://leetcode.com/problems/assign-elements-to-groups-with-constraints/
// 3447. Assign Elements to Groups with Constraints
pub struct Solution;
impl Solution {
    pub fn assign_elements(groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
        let mut m = std::collections::HashMap::new();
        for (i, &v) in elements.iter().enumerate() {
            m.entry(v).or_insert(i as i32);
        }
        let mut res = vec![i32::MAX; groups.len()];
        for i in 0..groups.len() {
            let g = groups[i];
            let mut j = 1;
            while j * j <= g {
                if g % j == 0 {
                    res[i] = res[i].min(*m.get(&j).unwrap_or(&i32::MAX));
                    res[i] = res[i].min(*m.get(&(g / j)).unwrap_or(&i32::MAX));
                }
                j += 1;
            }
            if res[i] == i32::MAX {
                res[i] = -1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assign_elements() {
        assert_eq!(
            Solution::assign_elements(vec![8, 4, 3, 2, 4], vec![4, 2]),
            vec![0, 0, -1, 1, 0]
        );
        assert_eq!(
            Solution::assign_elements(vec![2, 3, 5, 7], vec![5, 3, 3]),
            vec![-1, 1, 0, -1]
        );
        assert_eq!(
            Solution::assign_elements(vec![10, 21, 30, 41], vec![2, 1]),
            vec![0, 1, 0, 1]
        );
    }
}

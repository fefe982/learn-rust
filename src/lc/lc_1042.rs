// https://leetcode.com/problems/flower-planting-with-no-adjacent/
// 1042. Flower Planting With No Adjacent
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![1];
        }
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for p in paths {
            graph.entry(p[0] - 1).or_default().push(p[1] - 1);
            graph.entry(p[1] - 1).or_default().push(p[0] - 1);
        }
        let mut color = vec![0; n as usize];
        color[0] = 1;
        let mut idx = 1;
        while idx > 0 {
            color[idx] += 1;
            let mut good = true;
            if let Some(paths) = graph.get(&(idx as i32)) {
                for &adj in paths {
                    let adj = adj as usize;
                    if adj < idx && color[adj] == color[idx] {
                        good = false;
                        break;
                    }
                }
            }
            if good {
                idx += 1;
                if idx == color.len() {
                    break;
                }
                color[idx] = 0;
            }
        }
        color
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn garden_no_adj() {
        assert_eq!(
            Solution::garden_no_adj(
                3,
                [[1, 2], [2, 3], [3, 1]]
                    .into_iter()
                    .map(|x| x.to_vec())
                    .collect()
            ),
            vec![1, 2, 3]
        );
        assert_eq!(
            Solution::garden_no_adj(
                4,
                [[1, 2], [3, 4]].into_iter().map(|x| x.to_vec()).collect()
            ),
            vec![1, 2, 1, 2]
        );
        assert_eq!(
            Solution::garden_no_adj(
                4,
                [[1, 2], [2, 3], [3, 4], [4, 1], [1, 3], [2, 4]]
                    .into_iter()
                    .map(|x| x.to_vec())
                    .collect()
            ),
            vec![1, 2, 3, 4]
        );
    }
}

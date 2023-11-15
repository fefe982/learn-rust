// https://leetcode.com/problems/falling-squares/
// 699. Falling Squares
pub struct Solution;
impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(positions.len());
        let mut max_height = 0;
        let mut heights = std::collections::BTreeMap::new();
        for pos in positions {
            let px = pos[0];
            let py = pos[0] + pos[1];
            let ph = pos[1];
            let mut remove = vec![];
            let mut add = vec![];
            let mut max_base = 0;
            for (&r, &(l, h)) in heights.range(px + 1..) {
                if l < py {
                    max_base = max_base.max(h);
                    remove.push(r);
                    if l < px {
                        add.push((px, l, h));
                    }
                    if r > py {
                        add.push((r, py, h))
                    }
                } else {
                    break;
                }
            }
            for r in remove {
                heights.remove(&r);
            }
            for (r, l, h) in add {
                heights.insert(r, (l, h));
            }
            heights.insert(py, (px, ph + max_base));
            max_height = max_height.max(ph + max_base);
            ans.push(max_height);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_falling_squares() {
        assert_eq!(
            Solution::falling_squares(vec_vec![[1, 2], [2, 3], [6, 1]]),
            vec![2, 5, 5]
        );
        assert_eq!(
            Solution::falling_squares(vec_vec![[100, 100], [200, 100]]),
            vec![100, 100]
        );
    }
}

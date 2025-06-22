// https://leetcode.com/problems/the-skyline-problem/
// 218. The Skyline Problem
pub struct Solution;
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut bounds = std::collections::BTreeMap::<i32, Vec<i32>>::new();
        for vec in buildings {
            bounds.entry(vec[0]).or_default().push(vec[2]);
            bounds.entry(vec[1]).or_default().push(-vec[2]);
        }
        let mut skyline = Vec::new();
        let mut level = 0;
        let mut blds = std::collections::BinaryHeap::new();
        let mut remove = std::collections::BinaryHeap::new();
        for (x, hs) in bounds {
            for h in hs {
                if h > 0 {
                    blds.push(h);
                } else {
                    remove.push(-h);
                }
            }
            while !blds.is_empty() && !remove.is_empty() && blds.peek() == remove.peek() {
                blds.pop();
                remove.pop();
            }
            let new_level = if let Some(&l) = blds.peek() { l } else { 0 };
            if new_level != level {
                skyline.push(vec![x, new_level]);
                level = new_level;
            }
        }
        skyline
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn get_skyline() {
        assert_eq!(
            Solution::get_skyline(vec_vec![
                [2, 9, 10],
                [3, 7, 15],
                [5, 12, 12],
                [15, 20, 10],
                [19, 24, 8]
            ]),
            vec_vec![
                [2, 10],
                [3, 15],
                [7, 12],
                [12, 0],
                [15, 10],
                [20, 8],
                [24, 0]
            ]
        );
        assert_eq!(
            Solution::get_skyline(vec_vec![[0, 2, 3], [2, 5, 3]]),
            vec_vec![[0, 3], [5, 0]]
        );
    }
}

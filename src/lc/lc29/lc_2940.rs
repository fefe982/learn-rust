// https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet/
// 2940. Find Building Where Alice and Bob Can Meet
pub struct Solution;
impl Solution {
    fn set(v: &mut Vec<i32>, idx: usize, l: i32, r: i32, i: i32, val: i32) {
        if l + 1 == r {
            v[idx] = val;
            return;
        }
        let mid = (l + r) / 2;
        if i < mid {
            Self::set(v, idx * 2 + 1, l, mid, i, val);
        } else {
            Self::set(v, idx * 2 + 2, mid, r, i, val);
        }
        v[idx] = v[idx * 2 + 1].max(v[idx * 2 + 2]);
    }
    fn search(v: &Vec<i32>, idx: usize, l: i32, r: i32, i: i32, val: i32) -> i32 {
        if i >= r || v[idx] <= val {
            return -1;
        }
        if l + 1 == r {
            return l;
        }
        let mid = (l + r) / 2;
        let left = Self::search(v, idx * 2 + 1, l, mid, i, val);
        if left != -1 {
            return left;
        }
        return Self::search(v, idx * 2 + 2, mid, r, i, val);
    }
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = heights.len();
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        let mut v = vec![i32::MIN; sz * 2];
        for (i, &h) in heights.iter().enumerate() {
            Self::set(&mut v, 0, 0, sz as i32, i as i32, h);
        }
        let mut ans = vec![];
        for q in queries {
            let (mut l, mut r) = (q[0] as usize, q[1] as usize);
            if l > r {
                std::mem::swap(&mut l, &mut r);
            }
            if l == r || heights[r] > heights[l] {
                ans.push(r as i32);
                continue;
            }
            ans.push(Self::search(&v, 0, 0, sz as i32, r as i32, heights[l]));
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn leftmost_building_queries() {
        assert_eq!(
            Solution::leftmost_building_queries(
                vec![1, 2, 1, 2, 1, 2],
                vec_vec![
                    [0, 0],
                    [0, 1],
                    [0, 2],
                    [0, 3],
                    [0, 4],
                    [0, 5],
                    [1, 0],
                    [1, 1],
                    [1, 2],
                    [1, 3],
                    [1, 4],
                    [1, 5],
                    [2, 0],
                    [2, 1],
                    [2, 2],
                    [2, 3],
                    [2, 4],
                    [2, 5],
                    [3, 0],
                    [3, 1],
                    [3, 2],
                    [3, 3],
                    [3, 4],
                    [3, 5],
                    [4, 0],
                    [4, 1],
                    [4, 2],
                    [4, 3],
                    [4, 4],
                    [4, 5],
                    [5, 0],
                    [5, 1],
                    [5, 2],
                    [5, 3],
                    [5, 4],
                    [5, 5]
                ]
            ),
            vec![
                0, 1, 3, 3, 5, 5, 1, 1, -1, -1, -1, -1, 3, -1, 2, 3, 5, 5, 3, -1, 3, 3, -1, -1, 5, -1, 5, -1, 4, 5, 5,
                -1, 5, -1, 5, 5
            ]
        );
        assert_eq!(
            Solution::leftmost_building_queries(
                vec![6, 4, 8, 5, 2, 7],
                vec_vec![[0, 1], [0, 3], [2, 4], [3, 4], [2, 2]]
            ),
            vec![2, 5, -1, 5, 2]
        );
        assert_eq!(
            Solution::leftmost_building_queries(
                vec![5, 3, 8, 2, 6, 1, 4, 6],
                vec_vec![[0, 7], [3, 5], [5, 2], [3, 0], [1, 6]]
            ),
            vec![7, 6, -1, 4, 6]
        );
    }
}

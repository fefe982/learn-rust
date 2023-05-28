// https://leetcode.com/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows/
// 1439. Find the Kth Smallest Sum of a Matrix With Sorted Rows
pub struct Solution;
impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut minh = std::collections::BinaryHeap::<(i32, Vec<usize>)>::new();
        let mut maxh = std::collections::BinaryHeap::<i32>::new();
        let mut used = std::collections::HashSet::<Vec<usize>>::new();
        let m = mat.len();
        let n = mat[0].len();
        let mut sum = 0;
        for v in &mat {
            sum += v[0];
        }
        minh.push((-sum, vec![0; m]));
        maxh.push(sum);
        let k = k as usize;
        while let Some((cmin, min_pos)) = minh.pop() {
            if maxh.len() == k && -cmin >= *maxh.peek().unwrap() {
                continue;
            }
            for i in 0..m {
                if min_pos[i] + 1 < n {
                    let mut npos = min_pos.clone();
                    npos[i] += 1;
                    if used.contains(&npos) {
                        continue;
                    }
                    let nmin = cmin - mat[i][min_pos[i] + 1] + mat[i][min_pos[i]];
                    if maxh.len() == k && -nmin < *maxh.peek().unwrap() {
                        maxh.pop();
                    }
                    if maxh.len() < k {
                        used.insert(npos.clone());
                        maxh.push(-nmin);
                    }
                    if maxh.len() < k || -nmin < *maxh.peek().unwrap() {
                        minh.push((nmin, npos));
                    }
                }
            }
        }
        *maxh.peek().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn kth_smallest() {
        // assert_eq!(
        //     Solution::kth_smallest(vec_vec![[1, 3, 11], [2, 4, 6]], 5),
        //     7
        // );
        assert_eq!(
            Solution::kth_smallest(vec_vec![[1, 3, 11], [2, 4, 6]], 9),
            17
        );
        assert_eq!(
            Solution::kth_smallest(vec_vec![[1, 10, 10], [1, 4, 5], [2, 3, 6]], 7),
            9
        );
    }
}

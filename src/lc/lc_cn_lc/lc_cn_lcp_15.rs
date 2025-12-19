// https://leetcode.com/problems/you-le-yuan-de-mi-gong/
// LCP 15. 游乐园的迷宫
pub struct Solution;
impl Solution {
    pub fn visit_order(points: Vec<Vec<i32>>, direction: String) -> Vec<i32> {
        let direction = direction.as_bytes();
        let n = points.len();
        let mut ans = vec![];
        let mut visited = vec![false; n];
        let mut l = 0;
        for i in 1..n {
            if points[i][0] < points[l][0] {
                l = i;
            }
        }
        ans.push(l as i32);
        visited[l] = true;
        let cross = |p: usize, u: usize, v: usize| -> i32 {
            let v1 = (points[u][0] - points[p][0], points[u][1] - points[p][1]);
            let v2 = (points[v][0] - points[p][0], points[v][1] - points[p][1]);
            v1.0 * v2.1 - v1.1 * v2.0
        };
        for i in 0..n - 2 {
            let mut k = usize::MAX;
            for j in 0..n {
                if visited[j] {
                    continue;
                }
                if k == usize::MAX || ((cross(l, k, j) < 0) ^ (direction[i] == b'R')) {
                    k = j;
                }
            }
            ans.push(k as i32);
            visited[k] = true;
            l = k;
        }
        for i in 0..n {
            if !visited[i] {
                ans.push(i as i32);
                break;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(points: Vec<Vec<i32>>, direction: String) {
        let ans = Solution::visit_order(points.clone(), direction.clone());
        let mut visited = vec![false; points.len()];
        println!("{ans:?}");
        assert_eq!(ans.len(), points.len());
        let direction = direction.as_bytes();
        for i in 0..points.len() {
            assert!(ans[i] >= 0 && ans[i] < points.len() as i32);
            assert!(!visited[ans[i] as usize]);
            visited[ans[i] as usize] = true;
            if i > 0 && i < points.len() - 1 {
                let pre = ans[i - 1] as usize;
                let cur = ans[i] as usize;
                let next = ans[i + 1] as usize;
                let v1 = (points[pre][0] - points[cur][0], points[pre][1] - points[cur][1]);
                let v2 = (points[cur][0] - points[next][0], points[cur][1] - points[next][1]);
                if direction[i - 1] == b'R' {
                    assert!(v1.0 * v2.1 - v1.1 * v2.0 < 0);
                } else {
                    assert!(v1.0 * v2.1 - v1.1 * v2.0 > 0);
                }
            }
        }
    }
    #[test]
    fn visit_order() {
        check(vec_vec![[1, 1], [1, 4], [3, 2], [2, 1]], "LL".to_string());
        check(vec_vec![[1, 3], [2, 4], [3, 3], [2, 1]], "LR".to_string());
    }
}

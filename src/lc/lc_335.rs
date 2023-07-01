// https://leetcode.com/problems/self-crossing/
// 335. Self Crossing
pub struct Solution;
impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        let dir = [(0, 1), (-1, 0), (0, -1), (1, 0)];
        let (mut x, mut y) = (0, 0);
        let mut xmap = std::collections::BTreeMap::new();
        xmap.insert(0, (0, 0));
        let mut ymap = std::collections::BTreeMap::new();
        let mut xbound0 = 0;
        let mut xbound1 = 0;
        let mut ybound0 = 0;
        let mut ybound1 = 0;
        for (n, d) in distance.into_iter().enumerate() {
            let dd = dir[n % 4];
            let (nx, ny) = (x + dd.0 * d, y + dd.1 * d);
            if nx == x {
                let miny = y.min(ny);
                let maxy = y.max(ny);
                if xbound0 <= x && x <= xbound1 {
                    for (&yy, &(x1, x2)) in xmap.range(miny..=maxy) {
                        if yy != y && x1 <= x && x <= x2 {
                            return true;
                        }
                    }
                }
                ymap.insert(x, (miny, maxy));
                xbound0 = xbound0.min(x);
                xbound1 = xbound1.max(x);
            } else {
                let minx = x.min(nx);
                let maxx = x.max(nx);
                if ybound0 <= y && y <= ybound1 {
                    for (&xx, &(y1, y2)) in ymap.range(minx..=maxx) {
                        if xx != x && y1 <= y && y <= y2 {
                            return true;
                        }
                    }
                }
                xmap.insert(y, (minx, maxx));
                ybound0 = ybound0.min(y);
                ybound1 = ybound1.max(y);
            }
            x = nx;
            y = ny;
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_self_crossing() {
        assert_eq!(Solution::is_self_crossing(vec![1, 1, 2, 1, 1]), true);
        assert_eq!(Solution::is_self_crossing(vec![1, 1, 1, 1]), true);
        assert_eq!(Solution::is_self_crossing(vec![2, 1, 1, 2]), true);
        assert_eq!(Solution::is_self_crossing(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::is_self_crossing(vec![1, 1, 1, 2, 1]), true);
    }
}

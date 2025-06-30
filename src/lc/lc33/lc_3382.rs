// https://leetcode.com/problems/maximum-area-rectangle-with-point-constraints-ii/
// 3382. Maximum Area Rectangle with Point Constraints II
pub struct Solution;
impl Solution {
    fn add(ymax: &mut Vec<i32>, ymaxf: &mut Vec<i32>, mut yidx: usize, val: i32) {
        ymax[yidx] = ymax[yidx].max(val);
        while yidx < ymax.len() {
            ymaxf[yidx] = ymaxf[yidx].max(val);
            yidx += yidx & (!yidx + 1);
        }
    }
    fn find(ymax: &Vec<i32>, ymaxf: &Vec<i32>, ylow: usize, mut yhigh: usize) -> i32 {
        let mut res = i32::MIN;
        while ylow <= yhigh {
            let l = yhigh - (yhigh & (!yhigh + 1));
            if l + 1 >= ylow {
                res = res.max(ymaxf[yhigh]);
                yhigh = l;
            } else {
                res = res.max(ymax[yhigh]);
                yhigh -= 1;
            }
        }
        res
    }
    pub fn max_rectangle_area(x_coord: Vec<i32>, y_coord: Vec<i32>) -> i64 {
        let mut points = Vec::with_capacity(x_coord.len());
        let mut ymap = std::collections::BTreeMap::new();
        for (x, y) in x_coord.into_iter().zip(y_coord) {
            points.push((x, y));
            ymap.entry(y).or_default();
        }
        let mut ylen = 1;
        for (_, i) in ymap.iter_mut() {
            *i = ylen;
            ylen += 1;
        }
        let mut ymax = vec![i32::MIN; ylen];
        let mut ymaxf = vec![i32::MIN; ylen];
        points.sort();
        let mut res = -1;
        for i in 0..points.len() - 1 {
            let &yidx0 = ymap.get(&points[i].1).unwrap();
            if points[i].0 == points[i + 1].0 {
                let &yidx1 = ymap.get(&points[i + 1].1).unwrap();
                if ymax[yidx0] != i32::MIN
                    && ymax[yidx0] == ymax[yidx1]
                    && Self::find(&ymax, &ymaxf, yidx0 + 1, yidx1 - 1) < ymax[yidx0]
                {
                    res = res.max((points[i + 1].1 - points[i].1) as i64 * (points[i].0 - ymax[yidx0]) as i64);
                }
            }
            Self::add(&mut ymax, &mut ymaxf, yidx0, points[i].0);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_rectangle_area() {
        assert_eq!(Solution::max_rectangle_area(vec![1, 1, 3, 3], vec![1, 3, 1, 3]), 4);
        assert_eq!(
            Solution::max_rectangle_area(vec![1, 1, 3, 3, 2], vec![1, 3, 1, 3, 2]),
            -1
        );
        assert_eq!(
            Solution::max_rectangle_area(vec![1, 1, 3, 3, 1, 3], vec![1, 3, 1, 3, 2, 2]),
            2
        );
    }
}

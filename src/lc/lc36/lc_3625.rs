// https://leetcode.com/problems/count-number-of-trapezoids-ii/
// 3625. Count Number of Trapezoids II
pub struct Solution;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Decimal {
    x: i32,
    y: i32,
}
impl Decimal {
    fn new(x: i32, y: i32) -> Self {
        let g = Self::gcd(x, y);
        Self { x: x / g, y: y / g }
    }
    fn gcd(x: i32, y: i32) -> i32 {
        let mut xx = x.abs();
        let mut yy = y.abs();
        let g = loop {
            if yy == 0 {
                break xx;
            }
            xx = xx % yy;
            if xx == 0 {
                break yy;
            }
            yy = yy % xx;
        };
        if y < 0 || (y == 0 && x < 0) {
            -g
        } else {
            g
        }
    }
}

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let len = points.len();
        let mut slope_map = std::collections::HashMap::<Decimal, Vec<Decimal>>::new();
        let mut mid_map = std::collections::HashMap::<(i32, i32), Vec<Decimal>>::new();
        for i in 0..len {
            for j in i + 1..len {
                let (x1, y1) = (points[i][0], points[i][1]);
                let (x2, y2) = (points[j][0], points[j][1]);
                let dx = x1 - x2;
                let dy = y1 - y2;
                let slope = Decimal::new(dx, dy);
                let d = if dx == 0 {
                    Decimal::new(x1, 1)
                } else {
                    Decimal::new(y1 * dx - x1 * dy, dx)
                };
                slope_map.entry(slope).or_default().push(d);
                mid_map.entry((x1 + x2, y1 + y2)).or_default().push(slope);
            }
        }
        let mut ans = 0;
        for (_, b) in slope_map {
            if b.len() == 1 {
                continue;
            }
            let mut map_b = std::collections::HashMap::<Decimal, i32>::new();
            for d in b {
                *map_b.entry(d).or_default() += 1;
            }
            let mut sum = 0;
            for (_, v) in map_b {
                ans += sum * v;
                sum += v;
            }
        }
        for (_, b) in mid_map {
            if b.len() == 1 {
                continue;
            }
            let mut map_b = std::collections::HashMap::<Decimal, i32>::new();
            for d in b {
                *map_b.entry(d).or_default() += 1;
            }
            let mut sum = 0;
            for (_, v) in map_b {
                ans -= sum * v;
                sum += v;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_trapezoids() {
        assert_eq!(
            Solution::count_trapezoids(vec_vec![[-3, 2], [3, 0], [2, 3], [3, 2], [2, -3]]),
            2
        );
        assert_eq!(Solution::count_trapezoids(vec_vec![[0, 0], [1, 0], [0, 1], [2, 1]]), 1);
    }
}

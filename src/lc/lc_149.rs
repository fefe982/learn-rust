// https://leetcode.com/problems/max-points-on-a-line/
// 149. Max Points on a Line
pub struct Solution;
impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        let mut a = a.abs();
        let mut b: i32 = b.abs();
        while b != 0 {
            a = a % b;
            if a == 0 {
                return b;
            }
            b = b % a;
        }
        a
    }
    fn get_line(x1: i32, y1: i32, x2: i32, y2: i32) -> (i32, i32, i32) {
        let mut a = y2 - y1;
        let mut b = x1 - x2;
        let mut c = y2 * x1 - x2 * y1;
        if c < 0 {
            c = -c;
            b = -b;
            a = -a;
        } else if c == 0 && b < 0 {
            a = -a;
            b = -b;
        } else if c == 0 && b == 0 {
            a = a.abs();
        }
        let g = Self::gcd(Self::gcd(a, b), c);
        (a / g, b / g, c / g)
    }

    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 1 {
            return 1;
        }
        let mut base = std::collections::HashMap::<(i32, i32, i32), usize>::new();
        let mut cnt = std::collections::HashMap::<(i32, i32, i32), i32>::new();
        let mut max = 2;
        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let l = Self::get_line(points[i][0], points[i][1], points[j][0], points[j][1]);
                if let Some(&b) = base.get(&l) {
                    if b == i {
                        let c = cnt.entry(l).or_default();
                        *c += 1;
                        max = std::cmp::max(*c, max);
                    }
                } else {
                    base.insert(l, i);
                    cnt.insert(l, 2);
                }
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_points() {
        assert_eq!(Solution::max_points(vec_vec![[0,1],[0,0],[0,4],[0,-2],[0,-1],[0,3],[0,-4]]), 7);
        assert_eq!(Solution::max_points(vec_vec![[1, 1], [2, 2], [3, 3]]), 3);
        assert_eq!(
            Solution::max_points(vec_vec![[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]]),
            4
        );
    }
}

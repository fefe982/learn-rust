// https://leetcode.com/problems/rectangle-area-ii/
// 850. Rectangle Area II
pub struct Solution;
impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        let md = 1_000_000_007i64;
        let mut x = rectangles
            .iter()
            .map(|r| vec![r[0], r[2]].into_iter())
            .flatten()
            .collect::<Vec<_>>();
        x.sort_unstable();
        x = x
            .into_iter()
            .scan(i32::MIN, |state, v| {
                let s = v == *state;
                *state = v;
                Some((s, v))
            })
            .filter_map(|(s, v)| if s { None } else { Some(v) })
            .collect();
        let mut xmap = std::collections::HashMap::new();
        x.iter().enumerate().for_each(|(i, &x)| {
            xmap.insert(x, i);
        });
        let mut y = rectangles
            .iter()
            .map(|r| vec![(r[1], r[0], r[2], 1), (r[3], r[0], r[2], -1)].into_iter())
            .flatten()
            .collect::<Vec<_>>();
        y.sort_unstable();
        let mut last_y = 0;
        let mut sum = 0i64;
        let mut countx = vec![0; x.len()];
        for (vy, x0, x1, d) in y {
            let xl =
                countx
                    .iter()
                    .enumerate()
                    .take(x.len() - 1)
                    .fold(0, |acc, (i, &v)| if v > 0 { acc + x[i + 1] - x[i] } else { acc });
            sum = (sum + xl as i64 * (vy - last_y) as i64) % md;
            for i in *xmap.get(&x0).unwrap()..*xmap.get(&x1).unwrap() {
                countx[i] += d;
            }
            last_y = vy;
        }
        sum as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_rectangle_area() {
        assert_eq!(
            Solution::rectangle_area(vec_vec![[0, 0, 2, 2], [1, 0, 2, 3], [1, 0, 3, 1]]),
            6
        );
        assert_eq!(Solution::rectangle_area(vec_vec![[0, 0, 1000000000, 1000000000]]), 49);
    }
}

// https://leetcode.com/problems/erect-the-fence/
// 587. Erect the Fence
use std::cmp::Ordering;

pub struct Solution;
impl Solution {
    fn get_orgin(trees: &Vec<Vec<i32>>) -> (i32, i32) {
        const MAX: i32 = 200;
        let mut ymin = MAX;
        let mut xmin = MAX;
        for v in trees {
            if v[1] < ymin {
                ymin = v[1];
                xmin = v[0];
            } else if v[1] == ymin {
                xmin = std::cmp::min(xmin, v[0]);
            }
        }
        (xmin, ymin)
    }
    fn cmp_angle(x0: i32, y0: i32, a: &Vec<i32>, b: &Vec<i32>) -> Ordering {
        let dax = a[0] - x0;
        let day = a[1] - y0;
        let dbx = b[0] - x0;
        let dby = b[1] - y0;

        (day * dbx - dax * dby).cmp(&0)
    }
    pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (x0, y0) = Self::get_orgin(&trees);
        trees.sort_unstable_by(|a, b| {
            let dax = a[0] - x0;
            let day = a[1] - y0;
            let dbx = b[0] - x0;
            let dby = b[1] - y0;

            if dax == 0 && day == 0 {
                return Ordering::Less;
            }
            if dbx == 0 && dby == 0 {
                return Ordering::Greater;
            }

            let order = Self::cmp_angle(x0, y0, a, b);
            match order {
                Ordering::Equal => (dax * dax + day * day).cmp(&(dbx * dbx + dby * dby)),
                _ => order,
            }
        });
        let mut res = Vec::<Vec<i32>>::new();
        res.push(trees[0].clone());
        let mut start = true;
        for cur in 1..trees.len() {
            if start {
                res.push(trees[cur].clone())
            }
            if cur + 1 < trees.len() {
                if Self::cmp_angle(x0, y0, &trees[cur], &trees[cur + 1]) == Ordering::Equal {
                    continue;
                }
            }
            if start {
                start = false;
                continue;
            }
            while res.len() >= 2 {
                let last = res.len() - 1;
                let pre = res.len() - 2;
                if Self::cmp_angle(res[last][0], res[last][1], &res[pre], &trees[cur])
                    == Ordering::Less
                {
                    res.pop();
                } else {
                    break;
                }
            }
            res.push(trees[cur].clone())
        }
        if res.len() >= 3 {
            let last = res.len() - 1;
            let pre = res.len() - 2;
            if Self::cmp_angle(x0, y0, &res[last], &res[pre]) != Ordering::Equal {
                for tlast in (1..=trees.len() - 2).rev() {
                    match Self::cmp_angle(x0, y0, &res[last], &trees[tlast]) {
                        Ordering::Equal => {
                            res.push(trees[tlast].clone());
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
        }

        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn outer_trees() {
        assert_eq!(
            Solution::outer_trees(vec![
                vec![1, 1],
                vec![2, 2],
                vec![2, 0],
                vec![2, 4],
                vec![3, 3],
                vec![4, 2]
            ]),
            vec![vec![2, 0], vec![4, 2], vec![3, 3], vec![2, 4], vec![1, 1]]
        );
        assert_eq!(
            Solution::outer_trees(vec![vec![1, 2], vec![2, 2], vec![4, 2]]),
            vec![vec![1, 2], vec![2, 2], vec![4, 2]]
        );
    }
}

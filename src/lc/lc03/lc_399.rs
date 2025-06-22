// https://leetcode.com/problems/evaluate-division/
// 399. Evaluate Division
pub struct Solution;
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut cls = std::collections::HashMap::<String, usize>::new();
        let mut cls_arr = Vec::new();
        let mut empty_cls = Vec::new();
        for (e, &v) in equations.iter().zip(values.iter()) {
            let c0 = cls.get(&e[0]);
            let c1 = cls.get(&e[1]);
            match (c0, c1) {
                (None, None) => {
                    let new_cls = if empty_cls.len() == 0 {
                        cls_arr.push(std::collections::HashMap::new());
                        cls_arr.len() - 1
                    } else {
                        empty_cls.pop().unwrap()
                    };
                    cls_arr[new_cls].insert(e[1].clone(), 1.0);
                    cls_arr[new_cls].insert(e[0].clone(), v);
                    cls.insert(e[0].clone(), new_cls);
                    cls.insert(e[1].clone(), new_cls);
                }
                (None, Some(&c1)) => {
                    let &v1 = cls_arr[c1].get(&e[1]).unwrap();
                    cls_arr[c1].insert(e[0].clone(), v1 * v);
                    cls.insert(e[0].clone(), c1);
                }
                (Some(&c0), None) => {
                    let &v0 = cls_arr[c0].get(&e[0]).unwrap();
                    cls_arr[c0].insert(e[1].clone(), v0 / v);
                    cls.insert(e[1].clone(), c0);
                }
                (Some(&c0), Some(&c1)) => {
                    if c0 != c1 {
                        let mut m0 = std::collections::HashMap::new();
                        std::mem::swap(&mut m0, &mut cls_arr[c0]);
                        let m1 = &mut cls_arr[c1];
                        let &v1 = m1.get(&e[1]).unwrap();
                        let &v0 = m0.get(&e[0]).unwrap();
                        for (key, val) in m0.into_iter() {
                            m1.insert(key.clone(), val / v0 * v1 * v);
                            cls.insert(key, c1);
                        }
                        empty_cls.push(c0);
                    }
                }
            }
        }
        let mut res = Vec::new();
        for q in queries {
            let c0 = cls.get(&q[0]);
            let c1 = cls.get(&q[1]);
            res.push(match (c0, c1) {
                (Some(&c0), Some(&c1)) => {
                    if c0 == c1 {
                        let m = &cls_arr[c0];
                        m.get(&q[0]).unwrap() / m.get(&q[1]).unwrap()
                    } else {
                        -1.0
                    }
                }
                _ => -1.0,
            });
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn calc_equation() {
        assert_eq!(
            Solution::calc_equation(
                vec_vec_str![["a", "b"], ["b", "c"]],
                vec![2.0, 3.0],
                vec_vec_str![["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]]
            ),
            vec![6.0, 0.5, -1.0, 1.0, -1.0]
        );
        assert_eq!(
            Solution::calc_equation(
                vec_vec_str![["a", "b"], ["b", "c"], ["bc", "cd"]],
                vec![1.5, 2.5, 5.0],
                vec_vec_str![["a", "c"], ["c", "b"], ["bc", "cd"], ["cd", "bc"]]
            ),
            vec![3.75000, 0.40000, 5.00000, 0.20000]
        );
        assert_eq!(
            Solution::calc_equation(
                vec_vec_str![["a", "b"]],
                vec![0.5],
                vec_vec_str![["a", "b"], ["b", "a"], ["a", "c"], ["x", "y"]]
            ),
            vec![0.50000, 2.00000, -1.00000, -1.00000]
        );
    }
}

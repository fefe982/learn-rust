// https://leetcode.com/problems/perfect-rectangle/
// 391. Perfect Rectangle
pub struct Solution;
impl Solution {
    fn read_q(
        rectangles: &Vec<Vec<i32>>,
        qidx: &mut usize,
        q: &Vec<usize>,
        left: i32,
        chigh: &mut i32,
        qn: &mut Vec<usize>,
    ) -> bool {
        while *qidx < q.len() {
            if rectangles[q[*qidx]][2] < left {
                return false;
            } else if rectangles[q[*qidx]][2] == left {
                *qidx += 1;
            } else {
                if rectangles[q[*qidx]][1] < *chigh {
                    return false;
                } else if rectangles[q[*qidx]][1] == *chigh {
                    *chigh = rectangles[q[*qidx]][3];
                    qn.push(q[*qidx]);
                    *qidx += 1;
                } else {
                    break;
                }
            }
        }
        true
    }
    pub fn is_rectangle_cover(mut rectangles: Vec<Vec<i32>>) -> bool {
        rectangles.sort_unstable();
        let mut low = i32::MIN;
        let mut high = i32::MAX;
        let mut chigh = 0;
        let mut left = i32::MIN;
        let mut q = vec![];
        let mut qn = vec![];
        let mut qidx = 0;
        for (ir, r) in rectangles.iter().enumerate() {
            if left == i32::MIN {
                left = r[0];
                low = r[1];
                chigh = r[3];
                qn.push(ir);
            } else {
                if r[0] != left {
                    if !Self::read_q(&rectangles, &mut qidx, &q, left, &mut chigh, &mut qn) {
                        return false;
                    }
                    if qidx < q.len() {
                        return false;
                    }
                    q = qn;
                    qn = vec![];
                    if high != i32::MAX && chigh != high {
                        return false;
                    }
                    high = chigh;
                    chigh = low;
                    qidx = 0;
                    left = r[0];
                }
                if !Self::read_q(&rectangles, &mut qidx, &q, left, &mut chigh, &mut qn) {
                    return false;
                }
                if r[1] != chigh {
                    return false;
                }
                chigh = r[3];
                qn.push(ir);
            }
        }
        if !Self::read_q(&rectangles, &mut qidx, &q, left, &mut chigh, &mut qn) {
            return false;
        }
        if qidx < q.len() {
            return false;
        }
        let r = rectangles[qn[0]][2];
        for i in 1..qn.len() {
            if rectangles[qn[i]][2] != r {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn is_rectangle_cover() {
        assert_eq!(
            Solution::is_rectangle_cover(vec_vec![[0, 0, 1, 1], [0, 1, 3, 2], [1, 0, 2, 2]]),
            false
        );
        assert_eq!(
            Solution::is_rectangle_cover(vec_vec![
                [0, 0, 4, 1],
                [7, 0, 8, 2],
                [6, 2, 8, 3],
                [5, 1, 6, 3],
                [4, 0, 5, 1],
                [6, 0, 7, 2],
                [4, 2, 5, 3],
                [2, 1, 4, 3],
                [0, 1, 2, 2],
                [0, 2, 2, 3],
                [4, 1, 5, 2],
                [5, 0, 6, 1]
            ]),
            true
        );
        assert_eq!(
            Solution::is_rectangle_cover(vec_vec![
                [1, 1, 3, 3],
                [3, 1, 4, 2],
                [3, 2, 4, 4],
                [1, 3, 2, 4],
                [2, 3, 3, 4]
            ]),
            true
        );
        assert_eq!(
            Solution::is_rectangle_cover(vec_vec![
                [1, 1, 2, 3],
                [1, 3, 2, 4],
                [3, 1, 4, 2],
                [3, 2, 4, 4]
            ]),
            false
        );
        assert_eq!(
            Solution::is_rectangle_cover(vec_vec![
                [1, 1, 3, 3],
                [3, 1, 4, 2],
                [1, 3, 2, 4],
                [2, 2, 4, 4]
            ]),
            false
        );
    }
}

// https://leetcode.com/problems/minimum-domino-rotations-for-equal-row
// 1007. Minimum Domino Rotations For Equal Row
pub struct Solution;
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut found = false;
        let mut v = tops[0];
        let v1 = bottoms[0];
        let mut ctop = 1;
        let mut cbot = 0;
        if v == v1 {
            found = true;
            cbot = 1;
        }
        for i in 1..tops.len() {
            if found {
                if tops[i] == v {
                    ctop += 1;
                }
                if bottoms[i] == v {
                    cbot += 1;
                }
                if bottoms[i] != v && tops[i] != v {
                    return -1;
                }
            } else {
                if tops[i] == bottoms[i] {
                    if tops[i] == v {
                        ctop += 1;
                        cbot += 1;
                        found = true;
                    } else if tops[i] == v1 {
                        v = tops[i];
                        std::mem::swap(&mut ctop, &mut cbot);
                        ctop += 1;
                        cbot += 1;
                        found = true;
                    } else {
                        return -1;
                    }
                } else {
                    if tops[i] == v && bottoms[i] == v1 {
                        ctop += 1;
                    } else if tops[i] == v1 && bottoms[i] == v {
                        cbot += 1;
                    } else if tops[i] == v {
                        ctop += 1;
                        found = true;
                    } else if bottoms[i] == v {
                        cbot += 1;
                        found = true;
                    } else if tops[i] == v1 {
                        std::mem::swap(&mut ctop, &mut cbot);
                        v = v1;
                        ctop += 1;
                        found = true;
                    } else if bottoms[i] == v1 {
                        std::mem::swap(&mut ctop, &mut cbot);
                        v = v1;
                        cbot += 1;
                        found = true;
                    } else {
                        return -1;
                    }
                }
            }
        }
        tops.len() as i32 - ctop.max(cbot)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_domino_rotations() {
        assert_eq!(
            Solution::min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2]),
            2
        );
        assert_eq!(
            Solution::min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4]),
            -1
        );
    }
}

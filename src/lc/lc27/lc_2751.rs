// https://leetcode.com/problems/robot-collisions/
// 2751. Robot Collisions
pub struct Solution;
impl Solution {
    pub fn survived_robots_healths(positions: Vec<i32>, healths: Vec<i32>, directions: String) -> Vec<i32> {
        let mut robots = vec![];
        let directions = directions.as_bytes();
        for i in 0..positions.len() {
            robots.push((i, positions[i], healths[i], directions[i] == b'R'));
        }
        robots.sort_unstable_by_key(|r| r.1);
        let mut res: Vec<(usize, i32, i32, bool)> = vec![];
        for mut r in robots {
            while let Some(mut topr) = res.pop() {
                if !topr.3 || r.3 {
                    res.push(topr);
                    res.push(r);
                    r.2 = 0;
                    break;
                } else {
                    if topr.2 > r.2 {
                        topr.2 -= 1;
                        if topr.2 > 0 {
                            res.push(topr);
                        }
                        r.2 = 0;
                        break;
                    } else if topr.2 < r.2 {
                        r.2 -= 1;
                        if r.2 == 0 {
                            break;
                        }
                    } else {
                        r.2 = 0;
                        break;
                    }
                }
            }
            if r.2 > 0 {
                res.push(r);
            }
        }
        res.sort_unstable_by_key(|r| r.0);
        res.into_iter().map(|r| r.2).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_survived_robotes_healths() {
        assert_eq!(
            Solution::survived_robots_healths(vec![5, 4, 3, 2, 1], vec![2, 17, 9, 15, 10], "RRRRR".to_string()),
            vec![2, 17, 9, 15, 10]
        );
        assert_eq!(
            Solution::survived_robots_healths(vec![3, 5, 2, 6], vec![10, 10, 15, 12], "RLRL".to_string()),
            vec![14]
        );
        assert_eq!(
            Solution::survived_robots_healths(vec![1, 2, 5, 6], vec![10, 10, 11, 11], "RLRL".to_string()),
            vec![]
        );
    }
}

// https://leetcode.com/problems/asteroid-collision/
// 735. Asteroid Collision
pub struct Solution;
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        for a in asteroids {
            let mut app = a;
            while let Some(l) = res.pop() {
                if l > 0 && app < 0 {
                    if l.abs() == app.abs() {
                        app = 0;
                        break;
                    }
                    if l.abs() > app.abs() {
                        app = l;
                    }
                } else {
                    res.push(l);
                    res.push(app);
                    app = 0;
                    break;
                }
            }
            if app != 0 {
                res.push(app);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn asteroid_collision() {
        assert_eq!(
            Solution::asteroid_collision(vec![-2, -1, 1, 2]),
            vec![-2, -1, 1, 2]
        );
        assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
        assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
        assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
    }
}

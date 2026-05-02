// https://leetcode.com/problems/detect-squares/
// 2013. Detect Squares
struct DetectSquares {
    cnt: Vec<Vec<i32>>,
    points: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DetectSquares {
    fn new() -> Self {
        Self {
            cnt: vec![vec![0; 1001]; 1001],
            points: Vec::new(),
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        let (x, y) = (point[0] as usize, point[1] as usize);
        self.cnt[x][y] += 1;
        self.points.push((point[0], point[1]));
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let (x, y) = (point[0] as usize, point[1] as usize);
        self.points.iter().fold(0, |ans, &(px, py)| {
            if px == point[0] || py == point[1] || (point[0] - px).abs() != (point[1] - py).abs() {
                return ans;
            }
            ans + self.cnt[x][py as usize] * self.cnt[px as usize][y]
        })
    }
}

/**
 * Your DetectSquares object will be instantiated and called as such:
 * let obj = DetectSquares::new();
 * obj.add(point);
 * let ret_2: i32 = obj.count(point);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_detect_square() {
        let mut obj = DetectSquares::new();
        obj.add(vec![3, 10]);
        obj.add(vec![11, 2]);
        obj.add(vec![3, 2]);
        assert_eq!(obj.count(vec![11, 10]), 1);
        obj.add(vec![14, 8]);
        obj.add(vec![11, 2]);
        assert_eq!(obj.count(vec![11, 10]), 2);
    }
}

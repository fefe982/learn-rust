// https://leetcode.com/problems/student-attendance-record-ii/
// 552. Student Attendance Record II
pub struct Solution;
const MOD: i64 = 1000000007;
#[derive(Copy, Clone)]
struct IMod {
    i: i64,
}
impl IMod {
    fn new(i: i32) -> Self {
        Self { i: i as i64 }
    }
    fn from_i64(i: i64) -> Self {
        Self { i }
    }
    fn val(self) -> i32 {
        self.i as i32
    }
}
impl std::ops::Add<IMod> for IMod {
    type Output = IMod;
    fn add(self, rhs: IMod) -> IMod {
        IMod::from_i64((self.i + rhs.i) % MOD)
    }
}
impl std::ops::AddAssign<IMod> for IMod {
    fn add_assign(&mut self, rhs: IMod) {
        self.i = (self.i + rhs.i) % MOD;
    }
}
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let mut rec = vec![vec![IMod::new(0); 2]; 3];
        rec[0][0] = IMod::new(1);
        for _ in 0..n {
            let mut nrec = rec.clone();
            nrec[0][0] += rec[1][0] + rec[2][0];
            nrec[1][0] = rec[0][0];
            nrec[2][0] = rec[1][0];
            nrec[0][1] += rec[1][1] + rec[2][1] + rec[0][0] + rec[1][0] + rec[2][0];
            nrec[1][1] = rec[0][1];
            nrec[2][1] = rec[1][1];
            rec = nrec;
        }
        (rec[0][0] + rec[0][1] + rec[1][0] + rec[1][1] + rec[2][0] + rec[2][1]).val()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_record() {
        assert_eq!(Solution::check_record(2), 8);
        assert_eq!(Solution::check_record(1), 3);
        assert_eq!(Solution::check_record(10101), 183236316);
    }
}

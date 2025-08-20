// https://leetcode.com/problems/count-all-possible-routes/
// 1575. Count All Possible Routes
pub struct Solution;
const MOD: i64 = 1000000007;
#[derive(Copy, Clone)]
struct IMod {
    i: i64,
}
impl IMod {
    fn new(i: i64) -> Self {
        Self { i: i as i64 }
    }
    fn val(self) -> i32 {
        self.i as i32
    }
}
impl std::ops::AddAssign<IMod> for IMod {
    fn add_assign(&mut self, rhs: IMod) {
        return self.i = (self.i + rhs.i) % MOD;
    }
}
impl Solution {
    fn croute(
        locations: &Vec<i32>,
        start: usize,
        finish: usize,
        fuel: i32,
        cache: &mut Vec<Vec<Vec<IMod>>>,
    ) -> IMod {
        let el = locations[finish];
        let sl = locations[start];
        if el - sl > fuel {
            return IMod::new(0);
        }
        if cache[start][finish][fuel as usize].val() >= 0 {
            return cache[start][finish][fuel as usize];
        }
        let mut res = IMod::new(0);
        for i in (0..start).rev() {
            let il = locations[i];
            if sl - il + el - il > fuel {
                break;
            }
            res += Self::croute(locations, i, finish, fuel - (sl - il), cache);
        }
        for i in start + 1..finish {
            res += Self::croute(locations, i, finish, fuel - (locations[i] - sl), cache);
        }
        for i in finish..locations.len() {
            if i == start {
                res += IMod::new(1);
                continue;
            }
            let il = locations[i];
            if il - sl + il - el > fuel {
                break;
            }
            res += Self::croute(locations, finish, i, fuel - (il - sl), cache);
        }
        cache[start][finish][fuel as usize] = res;
        res
    }
    pub fn count_routes(mut locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let sl = locations[start as usize];
        let el = locations[finish as usize];
        locations.sort_unstable();
        let start = locations.binary_search(&sl).unwrap();
        let finish = locations.binary_search(&el).unwrap();
        Self::croute(
            &locations,
            start.min(finish),
            start.max(finish),
            fuel,
            &mut vec![
                vec![vec![IMod::new(-1); fuel as usize + 1]; locations.len()];
                locations.len()
            ],
        )
        .val()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_routes() {
        assert_eq!(Solution::count_routes(vec![2, 1, 5], 0, 0, 3), 2);
        assert_eq!(Solution::count_routes(vec![2, 3, 6, 8, 4], 1, 3, 5), 4);
        assert_eq!(Solution::count_routes(vec![4, 3, 1], 1, 0, 6), 5);
        assert_eq!(Solution::count_routes(vec![5, 2, 1], 0, 2, 3), 0);
    }
}

// https://leetcode.com/problems/process-restricted-friend-requests/
// 2076. Process Restricted Friend Requests
pub struct Solution;
impl Solution {
    fn get_p(p: &mut Vec<usize>, n: i32) -> usize {
        let mut i = n as usize;
        while p[i] != i {
            i = p[i];
        }
        p[n as usize] = i;
        i
    }
    pub fn friend_requests(n: i32, restrictions: Vec<Vec<i32>>, requests: Vec<Vec<i32>>) -> Vec<bool> {
        let mut p = (0..n as usize).collect::<Vec<_>>();
        let mut ans = vec![];
        'req: for r in requests {
            let (a, b) = (Self::get_p(&mut p, r[0]), Self::get_p(&mut p, r[1]));
            if a == b {
                ans.push(true);
                continue;
            }
            for res in &restrictions {
                let (ra, rb) = (Self::get_p(&mut p, res[0]), Self::get_p(&mut p, res[1]));
                if (a == ra && b == rb) || (a == rb && b == ra) {
                    ans.push(false);
                    continue 'req;
                }
            }
            p[a] = b;
            ans.push(true);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_friend_request() {
        assert_eq!(
            Solution::friend_requests(3, vec_vec![[0, 1]], vec_vec![[0, 2], [2, 1]]),
            [true, false]
        );
        assert_eq!(
            Solution::friend_requests(3, vec_vec![[0, 1]], vec_vec![[1, 2], [0, 2]]),
            [true, false]
        );
        assert_eq!(
            Solution::friend_requests(
                5,
                vec_vec![[0, 1], [1, 2], [2, 3]],
                vec_vec![[0, 4], [1, 2], [3, 1], [3, 4]]
            ),
            [true, false, true, false]
        );
    }
}

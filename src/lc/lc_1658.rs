// https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/
// 1658. Minimum Operations to Reduce X to Zero
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let n = nums.len();
        let mut v = vec![0];
        for num in nums {
            v.push(v.last().unwrap() + num);
        }
        let sum = *v.last().unwrap();
        if sum < x {
            return -1;
        }
        let mut min = i32::MAX;
        let mut i = 0;
        let mut j = n;
        while sum - v[j] < x {
            j -= 1;
        }
        while i <= j && j <= n {
            if v[i] + sum - v[j] == x {
                min = min.min((i + (n - j)) as i32);
                j += 1;
            }
            if j >= n {
                break;
            }
            while j < n && v[i] + sum - v[j] > x {
                j += 1;
            }
            while i < j && v[i] + sum - v[j] < x {
                i += 1;
            }
        }
        if min == i32::MAX {
            -1
        } else {
            min
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_capability() {
        assert_eq!(
            Solution::min_operations(
                vec![
                    5281, 4109, 5962, 1122, 8456, 7256, 9646, 1708, 7754, 3164, 8637, 3769, 5311,
                    3714, 2661, 3807, 5270, 2026, 164, 2510, 9151, 9796, 2563, 5234, 297, 2883,
                    5931, 8937, 9729, 5201, 9954, 6776, 5631, 2966, 1316, 3999, 2302, 8014, 7120,
                    4012, 9610, 6541, 7009, 3438, 9722, 4564, 9361, 5800, 5404, 9304, 1483, 882,
                    9834, 2592, 8812, 371, 7574, 8236, 2698, 7254, 4212, 9254, 5228, 9219, 5310,
                    6896, 1693, 9635, 1544, 5751, 7346, 4104, 4867, 977, 4988, 5981, 1644, 7039,
                    5808, 8544, 8961, 2364, 6156, 6468, 6357, 4003, 5106, 261, 3490, 7859, 2876,
                    9116, 3955, 1218, 2598, 5866, 1613, 3737, 8047, 8483, 2450, 9009, 9806, 8432,
                    8006, 616, 4519, 2180, 9641, 9988, 9151, 1902, 7002, 8397, 7355, 9236, 2302,
                    5218, 7151, 3728, 3118, 8120, 1728, 2963, 3588, 4535, 1953, 2923, 9496, 7013,
                    1692, 6697, 4962, 862, 3406, 3484, 9902, 3031, 6706, 4025, 6850, 5257, 7176,
                    6724, 1669, 6801, 9857, 8702, 1430, 8299, 3070, 5729, 3496, 9443, 6430, 5988,
                    7419, 4436, 9946, 7153, 6561, 9938, 6154, 8614, 1181, 9708, 2362, 7881, 9043,
                    1703, 5874, 1257, 5929, 5729, 606, 8534, 7245, 8213, 8227, 2170, 6374, 2483,
                    2111, 4013, 440, 6240, 6458, 3812, 15, 7167, 7967, 1863, 3034, 742, 7945, 1412,
                    8673, 6499, 8897, 4716, 816, 4596, 5337, 8502, 8974, 690, 9339, 2095, 4330,
                    7884, 8226, 6108, 8752, 8657, 1547, 7796, 4552, 1913, 3445, 9278, 5373, 7569,
                    1569, 4422, 2732, 2833, 5756, 7402, 1034, 8197, 8275, 5748, 8535, 6599, 2872,
                    9128, 3, 9739, 8336, 2451, 189, 3264, 878, 504, 2119, 4518, 2771, 4082, 6175,
                    4135, 5465, 8331, 7940, 2665, 6176, 7136, 5674, 4746, 6348
                ],
                151230
            ),
            28
        );
        assert_eq!(Solution::min_operations(vec![500, 1, 4, 2, 3], 500), 1);
        assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
        assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
        assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
    }
}

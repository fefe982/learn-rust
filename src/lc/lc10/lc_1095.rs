// https://leetcode.com/problems/find-in-mountain-array/
// 1095. Find in Mountain Array
pub struct Solution;

// This is the MountainArray's API interface.
// You should not implement it, or speculate about its implementation
pub struct MountainArray {
    v: Vec<i32>,
}
impl MountainArray {
    pub fn new(v: Vec<i32>) -> Self {
        Self { v }
    }
    fn get(&self, index: i32) -> i32 {
        self.v[index as usize]
    }
    fn length(&self) -> i32 {
        self.v.len() as i32
    }
}

impl Solution {
    pub fn find_in_mountain_array(target: i32, mountain_arr: &MountainArray) -> i32 {
        let mut low = 0;
        let mut high = mountain_arr.length() - 1;
        while low != high {
            let mid = (low + high) / 2;
            let vmid = mountain_arr.get(mid);
            let vmidl = mountain_arr.get(mid - 1);
            let vmidr = mountain_arr.get(mid + 1);
            if vmid > vmidl {
                low = mid;
            }
            if vmid > vmidr {
                high = mid;
            }
        }
        low = 0;
        let mut vlow = mountain_arr.get(low);
        let vhigh = mountain_arr.get(high);
        if vhigh == target {
            return high;
        }
        if vhigh < target {
            return -1;
        }
        if vlow == target {
            return 0;
        }
        if vlow < target {
            while low + 1 < high {
                let mid = (low + high) / 2;
                let vmid = mountain_arr.get(mid);
                if vmid == target {
                    return mid;
                } else if vmid < target {
                    low = mid;
                } else if vmid > target {
                    high = mid;
                }
            }
        }
        low = mountain_arr.length() - 1;
        vlow = mountain_arr.get(low);
        if vlow == target {
            return low;
        }
        if vlow > target {
            return -1;
        }
        while high + 1 < low {
            let mid = (low + high) / 2;
            let vmid = mountain_arr.get(mid);
            if vmid == target {
                return mid;
            } else if vmid < target {
                low = mid;
            } else if vmid > target {
                high = mid;
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_in_mountain_array() {
        assert_eq!(
            Solution::find_in_mountain_array(3, &MountainArray::new(vec![1, 2, 3, 4, 5, 3, 1])),
            2
        );
        assert_eq!(
            Solution::find_in_mountain_array(3, &MountainArray::new(vec![0, 1, 2, 4, 2, 1])),
            -1
        );
    }
}

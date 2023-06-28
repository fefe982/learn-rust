// https://www.codewars.com/kata/5518a860a73e708c0a000027
// Last digit of a huge number
fn check_zero(list: &[u64]) -> bool {
    if list[0] != 0 {
        false
    } else if list.len() == 1 {
        true
    } else {
        !check_zero(&list[1..])
    }
}
fn eval(list: &[u64], max: u64) -> u64 {
    if list.len() == 1 {
        return list[0].min(max);
    }
    if check_zero(&list[1..]) {
        return 1;
    }
    if list[0] == 0 {
        return 0;
    }
    if list[0] >= max {
        return max;
    }
    let mut nums = vec![];
    let mut n = list[0];
    while n < max {
        nums.push(n);
        n *= list[0];
    }
    nums.push(max);
    nums[eval(&list[1..], nums.len() as u64 - 1) as usize]
}
pub fn last_digit_base(list: &[u64], base: u64) -> u64 {
    if list.len() == 0 {
        return 1;
    }
    if list.len() == 1 {
        return list[0] % base;
    }
    if list[0] == 0 {
        return if check_zero(&list[1..]) { 1 } else { 0 };
    }
    let n = list[0] % base;
    if n == 0 || n == 1 {
        return n;
    }
    if check_zero(&list[1..]) {
        return 1;
    }
    let mut pn = n;
    let mut nums = vec![1];
    let mut used = vec![usize::MAX; base as usize];
    used[1] = 0;
    while used[pn as usize] == usize::MAX {
        used[pn as usize] = nums.len();
        nums.push(pn);
        pn = pn * n % base;
    }
    if used[pn as usize] == 1 {
        nums[0] = nums.pop().unwrap();
    } else if used[pn as usize] > 1 {
        let idx = eval(&list[1..], nums.len() as u64) as usize;
        if idx < nums.len() {
            return nums[idx];
        }
        let l = nums.len() - used[pn as usize];
        for idx in used[pn as usize]..nums.len() {
            nums[idx % l] = nums[idx];
        }
        nums.resize(l, 0);
    }
    if nums.len() == 1 {
        return nums[0];
    }
    return nums[last_digit_base(&list[1..], nums.len() as u64) as usize];
}
fn last_digit(list: &[u64]) -> u64 {
    last_digit_base(list, 10)
}
#[cfg(test)]
mod tests {
    use super::last_digit;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(v: &[u64], expected: u64) {
        assert_eq!(last_digit(v), expected, "{ERR_MSG} with list = {v:?}")
    }

    #[test]
    fn fixed_tests() {
        for (a, b) in [
            // (vec![], 1),
            // (vec![0, 0], 1),
            // (vec![0, 0, 0], 0),
            // (vec![1, 2], 1),
            // (vec![3, 4, 5], 1),
            // (vec![4, 3, 6], 4),
            // (vec![7, 6, 21], 1),
            // (vec![12, 30, 21], 6),
            (vec![2, 2, 2, 0], 4),
            (vec![2, 2, 101, 2], 6),
            (vec![937640, 767456, 981242], 0),
            (vec![123232, 694022, 140249], 6),
            (vec![499942, 898102, 846073], 6),
        ] {
            dotest(&a, b);
        }
    }
}

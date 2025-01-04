// https://leetcode.cn/problems/design-an-atm-machine/
// 2241. Design an ATM Machine
struct ATM {
    cnt: [i32; 5],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ATM {
    fn new() -> Self {
        Self { cnt: [0; 5] }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for i in 0..5 {
            self.cnt[i] += banknotes_count[i];
        }
    }

    fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        let mut res = vec![0; 5];
        let val = [20, 50, 100, 200, 500];
        let mut amount = amount;
        for i in (0..5).rev() {
            res[i] = (amount / val[i]).min(self.cnt[i]);
            amount -= res[i] * val[i];
        }
        if amount == 0 {
            for i in 0..5 {
                self.cnt[i] -= res[i];
            }
            res
        } else {
            vec![-1]
        }
    }
}

/**
 * Your ATM object will be instantiated and called as such:
 * let obj = ATM::new();
 * obj.deposit(banknotesCount);
 * let ret_2: Vec<i32> = obj.withdraw(amount);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2241() {
        let mut atm = ATM::new();
        atm.deposit(vec![0, 0, 1, 2, 1]);
        assert_eq!(atm.withdraw(600), vec![0, 0, 1, 0, 1]);
        atm.deposit(vec![0, 1, 0, 1, 1]);
        assert_eq!(atm.withdraw(600), vec![-1]);
        assert_eq!(atm.withdraw(550), vec![0, 1, 0, 0, 1]);
    }
}

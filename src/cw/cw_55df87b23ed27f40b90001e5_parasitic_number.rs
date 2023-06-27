// https://www.codewars.com/kata/55df87b23ed27f40b90001e5
// N-Parasitic Numbers Ending in N
fn get_number(str: &mut String, d: u8) {
    if d < 10 {
        str.push((b'0' + d as u8) as char);
    } else {
        str.push((b'A' + (d as u8 - 10)) as char);
    }
}
#[derive(Clone, Debug)]
struct BaseN {
    base: u8,
    n: Vec<u8>,
}
impl BaseN {
    fn new(base: u8, mut n: u32) -> BaseN {
        let mut v = vec![];
        while n > 0 {
            v.push((n % base as u32) as u8);
            n /= base as u32;
        }
        BaseN { base, n: v }
    }
    fn get_number(&self, str: &mut String) {
        for &d in self.n.iter().rev() {
            get_number(str, d);
        }
    }
}
impl std::ops::MulAssign<u8> for BaseN {
    fn mul_assign(&mut self, rhs: u8) {
        if self.n.len() == 0 {
            return;
        }
        if rhs == self.base {
            self.n.insert(0, 0);
        } else if rhs == 0 {
            self.n.clear();
        } else if rhs != 1 {
            let mut carry = 0;
            for i in 0..self.n.len() {
                let sum = self.n[i] as u32 * rhs as u32 + carry;
                self.n[i] = (sum % self.base as u32) as u8;
                carry = sum / self.base as u32
            }
            while carry > 0 {
                self.n.push((carry % self.base as u32) as u8);
                carry /= self.base as u32;
            }
        }
    }
}
impl std::ops::Mul<u8> for BaseN {
    type Output = BaseN;
    fn mul(self, rhs: u8) -> BaseN {
        let mut res = self;
        res *= rhs;
        res
    }
}
impl std::ops::Div<u32> for BaseN {
    type Output = Option<BaseN>;
    fn div(mut self, rhs: u32) -> Self::Output {
        let mut rem = 0;
        for i in (0..self.n.len()).rev() {
            let d = rem * self.base as u32 + self.n[i] as u32;
            self.n[i] = (d / rhs) as u8;
            rem = d % rhs;
            if i + 1 == self.n.len() && self.n[i] == 0 {
                self.n.remove(i);
            }
        }
        if rem == 0 {
            Some(self)
        } else {
            None
        }
    }
}
impl std::ops::Sub<u32> for BaseN {
    type Output = BaseN;
    fn sub(mut self, rhs: u32) -> Self::Output {
        let rhs = BaseN::new(self.base, rhs);
        let mut i = 0;
        let mut borrow = 0;
        while i < rhs.n.len() {
            if rhs.n[i] + borrow <= self.n[i] {
                self.n[i] -= rhs.n[i] + borrow;
                borrow = 0;
            } else {
                self.n[i] += self.base - (rhs.n[i] + borrow);
                borrow = 1;
            }
            i += 1;
        }
        while borrow > 0 {
            if borrow <= self.n[i] {
                self.n[i] -= borrow;
                borrow = 0;
            } else {
                self.n[i] += self.base - borrow;
                borrow = 1;
            }
            i += 1;
        }
        self
    }
}
fn calc_special(last_digit: u8, base: u8) -> String {
    let mut bn = BaseN::new(base, 1);
    let db1 = last_digit as u32 * base as u32 - 1;
    let d2 = last_digit as u32 * last_digit as u32;
    loop {
        bn *= base;
        let dnb = bn.clone() * last_digit - d2;
        if let Some(s) = dnb / db1 {
            if s.n.len() == bn.n.len() - 1 {
                let mut res = "".to_string();
                s.get_number(&mut res);
                get_number(&mut res, last_digit);
                return res;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::calc_special;

    #[test]
    fn sample_tests() {
        assert_eq!(calc_special(6, 8), "10127114202562304053446".to_string());
        assert_eq!(calc_special(4, 10), "102564".to_string());
        assert_eq!(calc_special(4, 16), "104".to_string());
    }
}

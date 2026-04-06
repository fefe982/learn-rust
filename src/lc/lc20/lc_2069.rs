// https://leetcode.com/problems/walking-robot-simulation-ii/
// 2069. Walking Robot Simulation II
pub struct Robot {
    width: i32,
    height: i32,
    perimeter: i32,
    steps: i32,
    moved: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            perimeter: (width + height - 2) * 2,
            steps: 0,
            moved: false,
        }
    }

    pub fn step(&mut self, num: i32) {
        self.steps = (self.steps + num % self.perimeter) % self.perimeter;
        self.moved = true;
    }

    pub fn get_pos(&self) -> Vec<i32> {
        let mut remain = self.steps;
        if remain <= self.width - 1 {
            return vec![remain, 0];
        }
        remain -= self.width - 1;

        if remain <= self.height - 1 {
            return vec![self.width - 1, remain];
        }
        remain -= self.height - 1;

        if remain <= self.width - 1 {
            return vec![self.width - 1 - remain, self.height - 1];
        }
        remain -= self.width - 1;

        vec![0, self.height - 1 - remain]
    }

    pub fn get_dir(&self) -> String {
        match self.steps {
            0 if self.moved => "South",
            0 => "East",
            s if s < self.width => "East",
            s if s < self.width + self.height - 1 => "North",
            s if s < self.width * 2 + self.height - 2 => "West",
            _ => "South",
        }
        .to_string()
    }
}

/**
 * Your Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut obj = Robot::new(6, 3);
        obj.step(2);
        obj.step(2);
        assert_eq!(obj.get_pos(), vec![4, 0]);
        assert_eq!(obj.get_dir(), "East");

        obj.step(2);
        obj.step(1);
        obj.step(4);
        assert_eq!(obj.get_pos(), vec![1, 2]);
        assert_eq!(obj.get_dir(), "West");
    }
}

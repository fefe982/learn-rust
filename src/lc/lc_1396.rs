// https://leetcode.com/problems/design-underground-system/
// 1396. Design Underground System
#[derive(Copy, Clone)]
struct TimeCnt {
    time: f64,
    cnt: i32,
}
impl Default for TimeCnt {
    fn default() -> Self {
        Self { time: 0.0, cnt: 0 }
    }
}
impl TimeCnt {
    fn add_time(&mut self, t: i32) {
        self.time += t as f64;
        self.cnt += 1;
    }
    fn get_time(&self) -> f64 {
        self.time / self.cnt as f64
    }
}
struct UndergroundSystem {
    stations: std::collections::HashMap<String, i32>,
    new_station_id: i32,
    sum_time: std::collections::HashMap<(i32, i32), TimeCnt>,
    check_ins: std::collections::HashMap<i32, (i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Self {
            stations: std::collections::HashMap::new(),
            new_station_id: 0,
            sum_time: std::collections::HashMap::new(),
            check_ins: std::collections::HashMap::new(),
        }
    }
    fn get_station_id(&mut self, station_name: String) -> i32 {
        if let Some(&id) = self.stations.get(&station_name) {
            id
        } else {
            let id = self.new_station_id;
            self.new_station_id += 1;
            self.stations.insert(station_name, id);
            id
        }
    }
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        let station_id = self.get_station_id(station_name);
        self.check_ins.insert(id, (station_id, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let station_id_out = self.get_station_id(station_name);
        let (station_id_in, time_in) = self.check_ins.remove(&id).unwrap();
        self.sum_time
            .entry((station_id_in, station_id_out))
            .or_default()
            .add_time(t - time_in);
    }

    fn get_average_time(&mut self, start_station: String, end_station: String) -> f64 {
        let station_id_in = self.get_station_id(start_station);
        let station_id_out = self.get_station_id(end_station);
        self.sum_time
            .get(&(station_id_in, station_id_out))
            .unwrap()
            .get_time()
    }
}
/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn underground() {
        let mut obj = UndergroundSystem::new();
        obj.check_in(45, String::from("Leyton"), 3);
        obj.check_in(32, String::from("Paradise"), 8);
        obj.check_in(27, String::from("Leyton"), 10);
        obj.check_out(45, String::from("Waterloo"), 15);
        obj.check_out(27, String::from("Waterloo"), 20);
        obj.check_out(32, String::from("Cambridge"), 22);
        assert_eq!(
            obj.get_average_time(String::from("Paradise"), String::from("Cambridge")),
            14.0
        );
        assert_eq!(
            obj.get_average_time(String::from("Leyton"), String::from("Waterloo")),
            11.0
        );
        obj.check_in(10, String::from("Leyton"), 24);
        assert_eq!(
            obj.get_average_time(String::from("Leyton"), String::from("Waterloo")),
            11.0
        );
        obj.check_out(10, String::from("Waterloo"), 38);
        assert_eq!(
            obj.get_average_time(String::from("Leyton"), String::from("Waterloo")),
            12.0
        );
    }
}

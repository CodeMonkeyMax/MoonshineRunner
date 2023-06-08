use super::{stat::Stat, sutil::*};

pub struct Car {
    name: String,
    spd: Stat,
    dur: Stat,
    cgo: Stat,
    inc: Stat,
    flavor: String,
}
impl Car {
    pub fn new() -> Self {
        Car {
            name: String::from(""),
            spd: Stat::new(0, 0),
            dur: Stat::new(0, 0),
            cgo: Stat::new(0, 0),
            inc: Stat::new(0, 0),
            flavor: String::from(""),
        }
    }
}
impl std::string::ToString for Car {
    fn to_string(&self) -> String {
        format!(
            "{}\n------------------------------\nSPD: {}\nDUR: {}\nCGO: {}\nINC: {}\n{}",
            self.name, self.spd, self.dur, self.cgo, self.inc, self.flavor
        )
    }
}

use super::{stat::Stat, sutil::*};
use crossterm::style::Stylize;

pub struct Car {
    pub(crate) name: String,
    pub(crate) spd: Stat,
    pub(crate) dur: Stat,
    pub(crate) cgo: Stat,
    pub(crate) inc: Stat,
    pub(crate) flavor: String,
    pub(crate) cargo_quality: u32,
    pub(crate) cargo_quantity: u32,
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
            cargo_quality: 0,
            cargo_quantity: 0,
        }
    }
}
impl std::string::ToString for Car {
    fn to_string(&self) -> String {
        format!(
            "{}\n------------------------------\nSPD: {} {}\nDUR: {} {}\nCGO: {} {}\nINC: {} {}\n{}",
            &self.name.to_string().green(),
            self.spd,
            self.spd.real,
            self.dur,
            self.dur.real,
            self.cgo,
            self.cgo.real,
            self.inc,
            self.inc.real,
            &self.flavor.to_string().red()
        )
    }
}

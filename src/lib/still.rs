use super::{stat::Stat, sutil::*};

pub struct Still {
    // Later, implement Vessel, Heat Source, Boiler, Condenser, and Barrels.
    pub spd: Stat,
    pub vol: Stat,
    pub qlt: Stat, // is there a better abbreviation for "quality?"
}
impl Still {
    pub fn new() -> Self {
        Still {
            spd: Stat::new(0, 0),
            vol: Stat::new(0, 0),
            qlt: Stat::new(0, 0),
        }
    }
}
impl std::string::ToString for Still {
    fn to_string(&self) -> String {
        format!(
            "Your Still:\n------------------------------\nSPEED: {} | VOLUME: {} | QUALITY: {}",
            self.spd, self.vol, self.qlt
        )
    }
}

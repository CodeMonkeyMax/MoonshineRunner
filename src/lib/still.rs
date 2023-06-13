use super::stat::Stat;

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
    pub fn default() -> Self {
        Still {
            spd: Stat::new(1, 12),
            vol: Stat::new(1, 12),
            qlt: Stat::new(1, 12),
        }
    }
}

impl std::string::ToString for Still {
    fn to_string(&self) -> String {
        format!(
            "| SPEED: {} | VOLUME: {} | QUALITY: {}",
            self.spd, self.vol, self.qlt
        )
    }
}

impl Clone for Still {
    fn clone(&self) -> Self {
        Self {
            spd: self.spd.clone(),
            vol: self.vol.clone(),
            qlt: self.qlt.clone(),
        }
    }
}

use super::sutil::*;
use crate::{CAR_STAT_LENGTH};

pub struct Stat {
    pub real: u32,
    pub max: u32,
}
impl Stat {
    pub fn new(real: u32, max: u32) -> Self {
        Stat { real, max }
    }
}
impl std::fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            progress_bar_upgradeable(self.real, self.max, CAR_STAT_LENGTH as u32)
        )
    }
}
impl Clone for Stat {
	fn clone(&self) -> Self {
		Self {
			real: self.real,
			max: self.max,
		}
}
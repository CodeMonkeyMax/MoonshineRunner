use super::sutil::*;
use crate::MAX_STAT;

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
            progress_bar_upgradeable(self.real, self.max, MAX_STAT)
        )
    }
}

use super::{car::Car, stat::Stat, still::Still, sutil::*};

pub struct Player {
    pub money: i32,
    pub car: Car,
    pub still: Still,
}
impl Player {
    pub fn new() -> Self {
        Player {
            money: 0,
            car: Car::new(),
            still: Still::new(),
        }
    }
}
impl std::string::ToString for Player {
    fn to_string(&self) -> String {
        format!("${}", self.money)
    }
}

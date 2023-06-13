use super::{car::Car, still::Still};

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
            still: Still::default(),
        }
    }
    pub fn get_money(&self) -> i32 {
        self.money
    }
    pub fn get_car(&self) -> Car {
        self.car.clone()
    }
		pub fn get_still(&self) -> Still {
				self.still.clone()
		}
impl std::string::ToString for Player {
    fn to_string(&self) -> String {
        format!("${}", self.money)
    }
}

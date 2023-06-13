use super::{car::*, stat::Stat, still::Still, wallet::Wallet};

pub struct Player {
    pub wallet: Wallet,
    pub car: Car,
    pub still: Still,
}
impl Player {
    pub fn new() -> Self {
        Self {
            wallet: Wallet { money: 1000 },
            car: Car::default_car(),
            still: Still::default(),
        }
    }
    pub fn get_money(&self) -> i32 {
        self.wallet.get()
    }
    //    pub fn set_money(&mut self, money: i32) {
    //        self.wallet.set(money);
    //    }
    pub fn get_car(&self) -> &Car {
        &self.car
    }
    //    pub fn set_car(&mut self, car: Car) {
    //        self.car = car;
    //    }
    pub fn get_still(&self) -> &Still {
        &self.still
    }
    //    pub fn set_still(&mut self, still: Still) {
    //        self.still = still;
    //    }
}

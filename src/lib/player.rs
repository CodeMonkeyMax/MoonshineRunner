use super::{car::*, stat::Stat, still::Still, wallet::Wallet};

pub struct Player {
    pub wallet: Wallet,
    pub car: Car,
    pub still: Still,
}
impl Player {
    pub fn new() -> Self {
        let mut player = Player::new();
        player.wallet.set(1000);
        player.car.name = "[F] Rusty Hatchback".to_string();
        player.car.spd = Stat::new(2, 5);
        player.car.dur = Stat::new(2, 5);
        player.car.cgo = Stat::new(8, 15);
        player.car.inc = Stat::new(2, 5);
        player.car.flavor = "Have you had your tetanus shots?".to_string();
        player
    }
    pub fn get_money(&self) -> i32 {
        self.wallet.get()
    }
    //    pub fn set_money(&mut self, money: i32) {
    //        self.wallet.set(money);
    //    }
    pub fn get_car(&self) -> Car {
        self.car.clone()
    }
    //    pub fn set_car(&mut self, car: Car) {
    //        self.car = car;
    //    }
    pub fn get_still(&self) -> Still {
        self.still.clone()
    }
    //    pub fn set_still(&mut self, still: Still) {
    //        self.still = still;
    //    }
}

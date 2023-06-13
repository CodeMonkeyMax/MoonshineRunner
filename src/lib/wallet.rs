pub struct Wallet {
    pub money: i32,
}
impl Wallet {
    pub fn new() -> Self {
        Wallet { money: 0 }
    }
    pub fn get(&self) -> i32 {
        self.money
    }
    pub fn set(&mut self, money: i32) {
        self.money = money;
    }
    pub fn add(mut self, money: i32) -> Self {
        self.money += money;
        self
    }
    pub fn sub(mut self, money: i32) -> Self {
        self.money -= money;
        self
    }
}

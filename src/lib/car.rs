use super::{stat::Stat, sutil::*};
use crate::MONEY_MULT;
use crossterm::style::Stylize;
use rand::Rng;

static S_DELTA: i32 = 3;
static A_DELTA: i32 = 2;
static B_DELTA: i32 = 1;
static C_DELTA: i32 = 0;
static D_DELTA: i32 = -1;
static F_DELTA: i32 = -2;

pub struct Car {
    pub(crate) name: String,
    pub(crate) spd: Stat,
    pub(crate) dur: Stat,
    pub(crate) cgo: Stat,
    pub(crate) inc: Stat,
    pub(crate) flavor: String,
    pub(crate) cargo_quality: u32,
    pub(crate) cargo_quantity: u32,
    pub(crate) class: char,
    pub(crate) price: u32,
    pub(crate) current_durability: u32,
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
            class: ' ',
            price: 0,
            current_durability: 0,
        }
    }
    pub fn generate(tier: u8) -> Car {
        let car_types = vec![
            "Commuter".to_string(),
            "Truck".to_string(),
            "Muscle Car".to_string(),
            "Hot Rod".to_string(),
            "Death Trap".to_string(),
            "Big Rig".to_string(),
        ];
        let prefixes = vec![
            "Rusty".to_string(),
            "Old".to_string(),
            "Slick".to_string(),
            "Large".to_string(),
            "Incognito".to_string(),
        ];
        let mut car = Car::new();
        let mut rng = rand::thread_rng();

        let car_type: String = match tier {
            1 => car_types
                .get(rng.gen_range(3..car_types.len() - 1))
                .unwrap_or(&"Bug".to_string())
                .to_string(),
            2 => car_types
                .get(rng.gen_range(1..3))
                .unwrap_or(&"Bug".to_string())
                .to_string(),
            3 => car_types
                .get(rng.gen_range(0..2))
                .unwrap_or(&"Bug".to_string())
                .to_string(),
            _ => panic!("Invalid tier"),
        };

        let roll = rng.gen_range(1..20);

        let prefix = prefixes
            .get(rng.gen_range(0..prefixes.len()))
            .unwrap_or(&"error-filled".to_string())
            .to_string();

        car.spd = match &car_type[..] {
            "Commuter" => Stat::new(rng.gen_range(2..=5), rng.gen_range(5..=6)),
            "Truck" => Stat::new(rng.gen_range(1..=4), rng.gen_range(4..=8)),
            "Muscle Car" => Stat::new(rng.gen_range(3..=6), rng.gen_range(6..=10)),
            "Hot Rod" => Stat::new(rng.gen_range(4..=7), rng.gen_range(7..=11)),
            "Death Trap" => Stat::new(rng.gen_range(5..=7), rng.gen_range(8..=12)),
            "Big Rig" => Stat::new(rng.gen_range(1..=3), rng.gen_range(3..=6)),
            _ => panic!("Invalid car type"),
        };
        car.dur = match &car_type[..] {
            "Commuter" => Stat::new(rng.gen_range(1..=3), rng.gen_range(3..=5)),
            "Truck" => Stat::new(rng.gen_range(3..=6), rng.gen_range(6..=11)),
            "Muscle Car" => Stat::new(rng.gen_range(2..=4), rng.gen_range(4..=9)),
            "Hot Rod" => Stat::new(rng.gen_range(2..=5), rng.gen_range(5..=6)),
            "Death Trap" => Stat::new(rng.gen_range(1..=3), rng.gen_range(3..=5)),
            "Big Rig" => Stat::new(rng.gen_range(5..=7), rng.gen_range(8..=12)),
            _ => panic!("Invalid car type"),
        };
        car.cgo = match &car_type[..] {
            "Commuter" => Stat::new(rng.gen_range(5..=15), rng.gen_range(15..=18)),
            "Truck" => Stat::new(rng.gen_range(15..=25), rng.gen_range(25..=40)),
            "Muscle Car" => Stat::new(rng.gen_range(10..=20), rng.gen_range(20..=28)),
            "Hot Rod" => Stat::new(rng.gen_range(4..=17), rng.gen_range(17..=23)),
            "Death Trap" => Stat::new(rng.gen_range(5..=7), rng.gen_range(7..=20)),
            "Big Rig" => Stat::new(rng.gen_range(18..=30), rng.gen_range(35..=60)),
            _ => panic!("Invalid car type"),
        };
        car.inc = match &car_type[..] {
            "Commuter" => Stat::new(rng.gen_range(6..=8), rng.gen_range(8..=12)),
            "Truck" => Stat::new(rng.gen_range(4..=6), rng.gen_range(6..=10)),
            "Muscle Car" => Stat::new(rng.gen_range(3..=5), rng.gen_range(5..=8)),
            "Hot Rod" => Stat::new(rng.gen_range(1..=2), rng.gen_range(3..=5)),
            "Death Trap" => Stat::new(rng.gen_range(1..=2), rng.gen_range(2..=3)),
            "Big Rig" => Stat::new(rng.gen_range(2..=4), rng.gen_range(4..=6)),
            _ => panic!("Invalid car type"),
        };
        car.class = match tier {
            1 => {
                if roll >= 16 {
                    'S'
                } else if roll >= 11 {
                    'A'
                } else {
                    'B'
                }
            }
            2 => {
                if roll >= 17 {
                    'A'
                } else if roll >= 11 {
                    'B'
                } else {
                    'C'
                }
            }
            3 => {
                if roll >= 17 {
                    'C'
                } else if roll >= 11 {
                    'D'
                } else {
                    'F'
                }
            }
            _ => panic!("Invalid tier"),
        };
        let delta;
        match car.class {
            'S' => delta = S_DELTA,
            'A' => delta = A_DELTA,
            'B' => delta = B_DELTA,
            'C' => delta = C_DELTA,
            'D' => delta = D_DELTA,
            'F' => delta = F_DELTA,
            _ => panic!("Invalid car class"),
        }
        car.spd.real = Self::safe_add(car.spd.real, delta);
        car.name = format!("[{}] {} {}", car.class, prefix, car_type);
        car.flavor = "TODO: More flavor text".to_string();
        println!(
            "10 + delta: {} | (10+delta)^2: {} | tier + 5: {}",
            delta + 10,
            ((delta + 10) * (delta + 10)),
            tier + 5
        );
        car.price = ((10 + delta) * (10 + delta) * (tier as i32) + 5) as u32;
        car.current_durability = car.dur.real;
        car
    }

    pub fn safe_add(a: u32, b: i32) -> u32 {
        let mut c = a as i32 + b;
        if c <= 0 {
            c = 1;
        }
        c as u32
    }
}
impl std::string::ToString for Car {
    fn to_string(&self) -> String {
        let durmessage = match self.current_durability {
            dur if dur == self.dur.real => self.dur.real.to_string().white(),
            dur if dur < self.dur.real => self.dur.real.to_string().bold().red(),
            _ => self.dur.real.to_string().bold().red(),
        };
        format!(
            "{}\n| SPD: {} {}\n| DUR: {} {}\n| CGO: {} {}\n| INC: {} {}\n| {}",
            &self.name.to_string().green(),
            self.spd,
            self.spd.real,
            self.dur,
            durmessage,
            self.cgo,
            self.cgo.real,
            self.inc,
            self.inc.real,
            &self.flavor.to_string().red()
        )
    }
}

use super::stat::Stat;
//use crate::MONEY_MULT;
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
    pub fn reset(&mut self) {
        let car = Self::default_car();
        self.name = car.name;
        self.spd = car.spd;
        self.dur = car.dur;
        self.cgo = car.cgo;
        self.inc = car.inc;
        self.flavor = car.flavor;
        self.class = car.class;
        self.cargo_quality = car.cargo_quality;
        self.cargo_quantity = car.cargo_quantity;
        self.price = car.price;
        self.current_durability = car.current_durability;
    }
    pub fn default_car() -> Car {
        Car {
            name: "[F] Rusty Hatchback".to_string(),
            spd: Stat::new(2, 5),
            dur: Stat::new(2, 5),
            cgo: Stat::new(8, 15),
            inc: Stat::new(2, 5),
            flavor: "Have you had your tetanus shots?".to_string(),
            class: 'F',
            cargo_quality: 0,
            cargo_quantity: 0,
            price: 0,
            current_durability: 0,
        }
    }
    pub fn generate(tier: u8) -> Car {
        let mut rng = rand::thread_rng();
        let car_class = make_class(tier);
        let car_type = make_type(car_class, tier);
        let car_prefix = make_prefix(tier);

        let mut car = Car::new();

        car.class = car_class;

        car.spd = make_speed(&car_class, &car_type);

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
        car.name = format!("[{}] {} {}", car.class, car_prefix, car_type);
        car.flavor = "TODO: More flavor text".to_string();
        car.price = ((10 + delta) * (10 + delta)) as u32 * (tier + 4) as u32;
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

fn make_speed(car_class: &char, car_type: &String) -> Stat {
    let rng = &mut rand::thread_rng();
    let car_speed = match &car_type[..] {
        "Commuter" => Stat::new(rng.gen_range(2..=5), rng.gen_range(5..=6)),
        "Truck" => Stat::new(rng.gen_range(1..=4), rng.gen_range(4..=8)),
        "Muscle Car" => Stat::new(rng.gen_range(3..=6), rng.gen_range(6..=10)),
        "Hot Rod" => Stat::new(rng.gen_range(4..=7), rng.gen_range(7..=11)),
        "Death Trap" => Stat::new(rng.gen_range(5..=7), rng.gen_range(8..=12)),
        "Big Rig" => Stat::new(rng.gen_range(1..=3), rng.gen_range(3..=6)),
        _ => panic!("Invalid car type"),
    };

    car_speed
}

fn make_class(tier: u8) -> char {
    let roll = rand::thread_rng().gen_range(1..20);
    match tier {
        1 => {
            if roll >= 16 {
                return 'S';
            } else if roll >= 11 {
                return 'A';
            } else {
                return 'B';
            }
        }
        2 => {
            if roll >= 17 {
                return 'A';
            } else if roll >= 11 {
                return 'B';
            } else {
                return 'C';
            }
        }
        3 => {
            if roll >= 17 {
                return 'C';
            } else if roll >= 11 {
                return 'D';
            } else {
                return 'F';
            }
        }
        _ => panic!("Invalid tier"),
    };
}

fn make_type(class: char, tier: u8) -> String {
    let mut rng = rand::thread_rng();
    let car_types = vec![
        "Commuter".to_string(),
        "Truck".to_string(),
        "Muscle Car".to_string(),
        "Hot Rod".to_string(),
        "Death Trap".to_string(),
        "Big Rig".to_string(),
    ];
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
    return car_type;
}

fn make_prefix(tier: u8) -> String {
    let prefixes = vec![
        "Rusty".to_string(),
        "Old".to_string(),
        "Slick".to_string(),
        "Large".to_string(),
        "Incognito".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
    ];
    return prefixes
        .get(rand::thread_rng().gen_range(0..prefixes.len()))
        .unwrap_or(&"error-filled".to_string())
        .to_string();
}

impl std::string::ToString for Car {
    fn to_string(&self) -> String {
        let mut durmessage = self.dur.real.to_string().white();
        if self.current_durability < self.dur.real {
            durmessage = format!("{}/{} - damaged!", self.current_durability, self.dur.real)
                .to_string()
                .bold()
                .red();
        }
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

impl Clone for Car {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            spd: self.spd.clone(),
            dur: self.dur.clone(),
            cgo: self.cgo.clone(),
            inc: self.inc.clone(),
            flavor: self.flavor.clone(),
            current_durability: self.current_durability,
            cargo_quality: self.cargo_quality,
            cargo_quantity: self.cargo_quantity,
            class: self.class,
            price: self.price,
        }
    }
}

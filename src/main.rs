use crossterm::style::Stylize;
use rand::Rng;
use std::io;

static MAX_STAT: u32 = 12;

// STAT #####################################################################################
pub struct Stat {
    real: u32,
    max: u32,
}
impl Stat {
    fn new(real: u32, max: u32) -> Self {
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

// CAR ######################################################################################
pub struct Car {
    name: String,
    spd: Stat,
    dur: Stat,
    cgo: Stat,
    inc: Stat,
    flavor: String,
}
impl Car {
    fn new() -> Self {
        Car {
            name: String::from(""),
            spd: Stat::new(0, 0),
            dur: Stat::new(0, 0),
            cgo: Stat::new(0, 0),
            inc: Stat::new(0, 0),
            flavor: String::from(""),
        }
    }
}
impl std::string::ToString for Car {
    fn to_string(&self) -> String {
        format!(
            "{}\n------------------------------\nSPD: {} | DUR: {} | CGO: {} | INC: {}\n{}",
            self.name, self.spd, self.dur, self.cgo, self.inc, self.flavor
        )
    }
}

// STILL ####################################################################################
pub struct Still {
    // Later, implement Vessel, Heat Source, Boiler, Condenser, and Barrels.
    spd: Stat,
    vol: Stat,
    qlt: Stat, // is there a better abbreviation for "quality?"
}
impl Still {
    fn new() -> Self {
        Still {
            spd: Stat::new(0, 0),
            vol: Stat::new(0, 0),
            qlt: Stat::new(0, 0),
        }
    }
}
impl std::string::ToString for Still {
    fn to_string(&self) -> String {
        format!(
            "Your Still:\n------------------------------\nSPEED: {} | VOLUME: {} | QUALITY: {}",
            self.spd, self.vol, self.qlt
        )
    }
}

// PLAYER ###################################################################################
pub struct Player {
    money: i32,
    car: Car,
    still: Still,
}
impl Player {
    fn new() -> Self {
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

// MAIN #####################################################################################
fn main() {
    println!("Hello, Gambler!");
    let mut player: Player = start();
    println!("⚀⚁⚂⚃⚄⚅!");
    println!("Quit at any time by answering \'q.\'");
    let mut quit = false;
    while !quit {
        let mut end_round = false;
        while !end_round {
            println!("Brew? (y/n)");
            if get_valid_input(&['y', 'n']).unwrap() == 'y' {
                brew(&mut player);
            }
            println!("Drive? (y/n)");
            if get_valid_input(&['y', 'n']).unwrap() == 'y' {
                drive(&mut player);
            }
            println!("Barter? (y/n)");
            if get_valid_input(&['y', 'n']).unwrap() == 'y' {
                barter(&mut player);
            }
            println!("Buy? (y/n)");
            if get_valid_input(&['y', 'n']).unwrap() == 'y' {
                barter(&mut player);
            }

            if player.money >= 100000 {
                println!("You won!");
                end_round = true;
                quit = true;
            }
        }
    }
}

fn start() -> Player {
    let mut player = Player::new();
    player.money = 0;
    player
}

fn get_random_number(_d: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(0.._d) + 1;
    return random_number;
}

// GAMEPLAY LOOP STARTS HERE! ###################################################################################
fn brew(mut player: &mut Player) {
    //take into account player's still size and quality
    let _ = get_random_number(3); // This does nothing. Remove it later.
    println!("You brewed, for instance, \"OKAY HOOCH\"");
}

fn drive(mut player: &mut Player) {
    // Give route options
    // get input
    // start a for/while countdown. Nice n easy. Remember that we'll be counting down distance AND rolls. Pick one.
    for i in 0..5 {
        print!("Progress: {}/5\r", i + 1);
        for j in 0..i {
            println!("Line {}: {}\r", j + 1, " ".repeat(j));
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("You made it! Just in the nick of time, too!");
}

fn barter(mut player: &mut Player) {
    player.money += 10;
}

fn buy(mut player: &mut Player) {
    player.money -= 5;
}

/// Prompts user for input - will only return if input is valid. Quits on `q`.
/// # Arguments
/// * `chars` - All valid inputs
fn get_valid_input(chars: &[char]) -> Option<char> {
    let mut answer: char = ' ';
    let mut input_is_valid: bool = false;
    while !input_is_valid {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Line should not be blank.");

        let input = input.to_lowercase().chars().next();

        if input.unwrap() == 'q' {
            println!("Sure, why would you want to play a fun game with me anyway?\nOh, no, I'm sure you have more important things to do.");
            std::process::exit(0);
        }

        for c in chars {
            if *c == input.unwrap() {
                answer = *c;
                input_is_valid = true;
            }
        }
        if answer == ' ' {
            println!("Let's try that again.");
        }
    }

    Some(answer)
}

fn progress_bar(real: u32, max: u32) -> String {
    let mut result: String = String::from("[");
    for _i in 0..real {
        result.push('=');
    }
    for _i in real..max {
        result.push(' ');
    }
    result.push(']');
    result
}
fn progress_bar_upgradeable(real: u32, max: u32, scale: u32) -> String {
    let mut result: String = String::from("[");
    for _i in 0..real {
        result.push('=');
    }
    for _i in real..max {
        result.push('-');
    }
    for _i in real..scale {
        result.push(' ');
    }
    result.push(']');
    result
}
fn progress_bar_head(real: u32, max: u32) -> String {
    let mut result: String = String::from("[");
    for _i in 0..real - 1 {
        result.push('=');
    }
    result.push('#');
    for _i in real..max {
        result.push('-');
    }
    result.push(']');
    result
}

use crossterm::style::Stylize;
use lib::{car::Car, player::Player, stat::Stat, still::Still, sutil};
use rand::Rng;
use std::io;

pub mod lib;

pub static MAX_STAT: u32 = 12;

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
                buy(&mut player);
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
    println!("{}", "# STAGE 1 of 4: BREW \t\t#####".yellow());
    println!("Alright gambler, let's brew some backyard shine.");
    println!("{}", player.still.to_string());
    //take into account player's still size and quality
    let die = get_random_number(3); // This does nothing. Remove it later.
    println!("You brewed, for instance, \"OKAY HOOCH\"");
}

fn drive(mut player: &mut Player) {
    let route1 = 24;
    let route2 = 36;
    let route3 = 48;
    let route4 = 36;
    println!("{}", "# STAGE 2 of 4: DRIVE \t\t#####".yellow());
    println!("Buckle up, Gambler. It's time to drive!");
    println!("Your Car:\n{}", player.car.to_string());
    // Give route options
    // Each route has: distance, roll countdown, 'heat' (cops base roll/roll modifier), and price & preference
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
    println!("{}", "# STAGE 3 of 4: BARTER \t#####".yellow());
    player.money += 10;
}

fn buy(mut player: &mut Player) {
    println!("{}", "# STAGE 4 of 4: BUY \t\t#####".yellow());
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

use crossterm::style::Stylize;
use lib::{car::Car, player::Player, route::Route, stat::Stat, still::Still, sutil};
use rand::Rng;
use std::io;

pub mod lib;

pub static MAX_STAT: u32 = 12;

// MAIN #####################################################################################
fn main() {
    println!("Hello, Gambler!");
    let mut player: Player = start();

    // initialization done. Do your tesing here.

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
    let quality = get_random_number(3);
    match quality {
        1 => {
            println!(
                "Yeesh! You brewed some '{}' Good luck getting rid of this!",
                "Rotgut Wiskee".red()
            );
        }
        2 => {
            println!(
                "You brewed some '{}.' This will be plenty easy to move!",
                "OK Hooch".yellow()
            );
        }
        3 => {
            println!(
                "Hoo wee! This here's some '{}!' Worth top dollar, to the right buyer...",
                "White Lightning".green()
            )
        }
        _ => panic!("{}", "BAD NUMBER!".red()),
    }
}

fn drive(mut player: &mut Player) {
    let routes: Vec<Route> = vec![
        // Route 1
        Route {
            name: String::from("Route 1"),
            distance: 24,
            heat: 0,
            prefereces: vec![4, 3, 2],
            prices: vec![5, 3, 2],
        },
        // Route 2
        Route {
            name: String::from("Route 2"),
            distance: 24,
            heat: 0,
            prefereces: vec![4, 3, 2],
            prices: vec![5, 3, 2],
        },
        // Route 3
        Route {
            name: String::from("Route 3"),
            distance: 24,
            heat: 0,
            prefereces: vec![4, 3, 2],
            prices: vec![5, 3, 2],
        },
        // Route 4
        Route {
            name: String::from("Route 4"),
            distance: 24,
            heat: 0,
            prefereces: vec![4, 3, 2],
            prices: vec![5, 3, 2],
        },
        // Route 5
        Route {
            name: String::from("Route 5"),
            distance: 24,
            heat: 0,
            prefereces: vec![4, 3, 2],
            prices: vec![5, 3, 2],
        },
        // Route 6
        Route {
            name: String::from("Route 6"),
            distance: 24,
            heat: 0,
            prefereces: vec![4, 3, 2],
            prices: vec![5, 3, 2],
        },
    ];
    let mut chosen_routes: Vec<usize> = Vec::new();
    println!("{}", "# STAGE 2 of 4: DRIVE \t\t#####".yellow());
    println!("Buckle up, Gambler. It's time to drive!");
    println!("Your Car:\n{}", player.car.to_string());
    while chosen_routes.len() < 3 {
        let i: usize = get_random_number((routes.len() - 1) as i32) as usize;
        if !chosen_routes.contains(&i) {
            chosen_routes.push(i as usize);
        }
    }
    println!("Available Routes:\n");
    for route_index in chosen_routes {
        println!("{}\n", routes[route_index]);
    }
    // Give route options
    // Each route has: distance, roll countdown, 'heat' (cops base roll/roll modifier), and price & preference
    // get input
    // start a for/while countdown. Nice n easy. Remember that we'll be counting down distance AND rolls. Pick one.
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

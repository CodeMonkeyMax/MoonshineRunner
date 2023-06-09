use crossterm::event::{self, KeyCode, KeyEvent};
use crossterm::style::Stylize;
use lib::{car::Car, player::Player, route::Route, stat::Stat, still::Still, sutil::*};
use rand::Rng;
use std::io;

pub mod lib;

pub static MAX_STAT: u32 = 12;
pub static CAR_STAT_LENGTH: u8 = 12;
pub static MONEY_MULT: f64 = 10.0;

////////////////////////////////////////////////////////////////////////////////////////
// MAIN ////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////

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
            // BREW
            prompt_to_continue(Some("brew stage".to_string()));
            brew(&mut player);

            // DRIVE
            prompt_to_continue(Some("drive stage".to_string()));
            let mut cargo_status: i32 = 0;
            cargo_status = drive(&mut player);
            // We leave here with a number that, compared to player's car durability, gives us a damage level. The car and cargo quality/quantity are all accessible through player.car.

            //BARTER
            if cargo_status > 0 {
                prompt_to_continue(Some("barter stage".to_string()));
                barter(&mut player, cargo_status);
            } else if cargo_status < 0 {
                // player is boned
                break;
            }

            // BUY
            prompt_to_continue(Some("buy stage".to_string()));
            buy(&mut player);

            if player.money >= 100000 {
                println!("You won!");
                end_round = true;
                quit = true;
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
// TIER 2  ////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////

fn start() -> Player {
    let mut player = Player::new();
    player.money = 0;
    player.car.name = "Rusty Hatchback".to_string();
    player.car.spd = Stat::new(2, 5);
    player.car.dur = Stat::new(2, 5);
    player.car.cgo = Stat::new(8, 15);
    player.car.inc = Stat::new(2, 5);
    player.car.flavor = "Have you had your tetanus shots?".to_string();
    player
}

fn brew(mut player: &mut Player) {
    println!("{}", "# STAGE 1 of 4: BREW \t#####".yellow());
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
            player.car.cargo_quantity = player.still.vol.real;
            player.car.cargo_quality = 1;
        }
        2 => {
            println!(
                "You brewed some '{}.' This will be plenty easy to move!",
                "OK Hooch".yellow()
            );
            player.car.cargo_quantity = player.still.vol.real;
            player.car.cargo_quality = 2;
        }
        3 => {
            println!(
                "Hoo wee! This here's some '{}!' Worth top dollar, to the right buyer...",
                "White Lightning".green()
            );
            player.car.cargo_quantity = player.still.vol.real;
            player.car.cargo_quality = 1;
        }
        _ => panic!("{}", "BAD NUMBER!".red()),
    }
}

fn drive(mut player: &mut Player) -> i32 {
    println!("{}", "# STAGE 2 of 4: DRIVE \t#####".yellow());
    println!("Buckle up, Gambler. It's time to drive!");
    println!("Your Car:\n{}", player.car.to_string());
    let route = choose_route();

    let route_distance = route.distance;
    let heat = route.heat;
    // start a for/while countdown. Nice n easy.
    let mut distance_traveled: u32 = 0;
    let player_still_alive = true;

    println!("Get ready, gambler!");
    while distance_traveled < route_distance && player_still_alive {
        println!("{}", progress_bar_head(distance_traveled, route_distance));
        println!("Press ENTER to roll...");
        wait_for_enter();
        let die1: u8 = get_random_number(6) as u8;
        let die2: u8 = get_random_number(6) as u8;
        let police_roll: u8 = get_random_number(6) as u8;
        let mut player_spotted = false;

        println!(
            "You Rolled:\tDie 1\tDie 2\n\t\t\t {}[{}]\t {}[{}]",
            die1,
            die_from_u8(die1),
            die2,
            die_from_u8(die2)
        );

        println!("You can use one roll to go faster, and one to drive more stealthily.");
        println!("Which roll do you want to apply to your SPEED? (1/2)");
        println!(
            "Die 1: [{}] {} + {} -> {} Total Speed",
            die1,
            die_from_u8(die1),
            player.car.spd.real,
            die1 as u32 + player.car.spd.real
        );
        println!(
            "Die 2: [{}] {} + {} -> {} Total Speed",
            die2,
            die_from_u8(die2),
            player.car.spd.real,
            die2 as u32 + player.car.spd.real
        );

        let other_die: u8;
        let die_choice = get_valid_input(&['1', '2']).unwrap().to_digit(10);
        match die_choice.unwrap() {
            1 => {
                distance_traveled += die1 as u32 + player.car.spd.real;
                other_die = die2;
                1
            }
            2 => {
                distance_traveled += die2 as u32 + player.car.spd.real;
                other_die = die1;
                2
            }
            _ => {
                panic!("BAD NUMBER!");
            }
        };

        if distance_traveled > route_distance {
            distance_traveled = route_distance;
        }
        println!("{}", progress_bar_head(distance_traveled, route_distance));

        println!(
            "Cops' Roll:\t\t\t[{}] {} + {} -> {} Total Heat",
            die_from_u8(police_roll),
            police_roll,
            heat,
            police_roll + heat as u8
        );
        println!(
            "Your stealth roll: \t[{}] {} + {} -> {} total incognito score",
            die_from_u8(other_die),
            other_die,
            player.car.inc.real,
            other_die as u32 + player.car.inc.real
        );
        let total_heat: u32 = police_roll as u32 + heat;
        if other_die as u32 + player.car.inc.real < total_heat {
            println!("You've been made! Floor it!");
            prompt_to_continue(Some("roll".to_string()));
            player_spotted = true;
        } else {
            println!("Good work, gambler. They haven't spotted you yet.");
        }
        if player_spotted {
            if distance_traveled == route_distance {
                println!("Ha! You squeaked right by 'em!");
                return player.car.dur.real as i32 + 1;
            }
            let chase_result = chase(&mut player, route_distance, distance_traveled, heat);
            match chase_result {
                -1 => {
                    println!("Your whiskey and your ride are history, Gambler!");
                }
                0 => {
                    println!("You barely made it outta there! Good thing you talked 'em into returning your keys!'");
                }
                _ => {
                    println!("Holy Hockey Sticks! You made it by the seat of your pants!");
                }
            }
            return chase_result;
        }
    }
    println!("You made it!");
    return player.car.dur.real as i32 + 1;
}

fn barter(mut player: &mut Player, cargo_status: i32) {
    let mut mult: f64 = 1.0;
    println!("{}", "# STAGE 3 of 4: BARTER \t#####".yellow());
    match cargo_status {
        -1 => panic!("Player has negative cargo status - should not be in 'barter()' at all!"),
        0 => {
            mult = 0.5;
        }
        _ => {
            mult = (cargo_status as f64 / (2.0 * player.car.dur.real as f64)) + 0.5;
        }
    }
    println!("DEBUG: Mult = {}", mult);
    let die = get_random_number(6);
    let money_increment = match die {
        1 => mult * 0.65 * cargo_status as f64 * player.car.cargo_quality as f64,
        2 => mult * 0.75 * cargo_status as f64 * player.car.cargo_quality as f64,
        3 => mult * 0.85 * cargo_status as f64 * player.car.cargo_quality as f64,
        4 => mult * 0.95 * cargo_status as f64 * player.car.cargo_quality as f64,
        5 => mult * 1.0 * cargo_status as f64 * player.car.cargo_quality as f64,
        6 => mult * 1.15 * cargo_status as f64 * player.car.cargo_quality as f64,
        _ => panic!("The die has died and turned into nonsense. Seek professional help."),
    };
    println!(
        "You rolled a {}. Given this and the results of your drive, you get: ${}",
        die, money_increment as i32
    );
    player.money += money_increment as i32;
}

fn buy(mut player: &mut Player) {
    println!("{}", "# STAGE 4 of 4: BUY \t#####".yellow());
    println!(
        "Your Car: {}\nYour Still: {}",
        player.car.to_string(),
        player.still.to_string()
    );
    println!("Do you want to buy any upgrades? (y/n)");
    if get_valid_input(&['y', 'n']) == Some('y') {
        println!("Do you want to buy upgrades for your CAR or for your STILL? (c/s)");
        match get_valid_input(&['c', 's']) {
            Some('c') => {
                println!("OK, let's look at some car upgrades.");
                println!("Do you want to upgrade SPEED, DURABILITY or CARGO? (s/d/c)");
                match get_valid_input(&['s', 'đ', 'c']) {
                    Some('s') => {
                        player.car.spd.real += 1;
                        player.money -= 10;
                    }
                    Some('d') => {
                        player.car.dur.real += 1;
                        player.money -= 10;
                    }
                    Some('c') => {
                        player.car.cgo.real += 3;
                        player.money -= 10;
                    }
                    Some(c) => {
                        panic!("How did you get {}?", c);
                    }
                    None => {
                        panic!("This isn't supposed to happen.");
                    }
                }
            }
            Some('s') => {
                println!("OK, let's look at some still upgrades.");
                println!("Do you want to upgrade VOLUME, SPEED or QUALITY? (v/s/q)");
                match get_valid_input(&['v', 's', 'q']) {
                    Some('v') => {
                        player.still.vol.real += 1;
                        player.money -= 10;
                    }
                    Some('s') => {
                        player.still.spd.real += 1;
                        player.money -= 10;
                    }
                    Some('q') => {
                        player.still.qlt.real += 1;
                        player.money -= 10;
                    }
                    Some(c) => {
                        panic!("How did you get {}?", c);
                    }
                    None => {
                        panic!("This isn't supposed to happen.");
                    }
                }
            }
            Some(c) => {
                panic!("How did you get {}?", c);
            }
            None => {
                panic!("This isn't supposed to happen.");
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
// TIER 3  ////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////

fn chase(
    mut player: &mut Player,
    route_distance: u32,
    mut distance_traveled: u32,
    heat: u32,
) -> i32 {
    // here's where I have to decide what the formula should be. Maybe 1 roll for every 3-4 units of distance left? That's what's fair, as 3.5 is the average die roll.
    let mut num_rolls_left = (route_distance - distance_traveled) / (player.car.spd.real + 4);
    if num_rolls_left == 0 {
        num_rolls_left = 1;
    }
    let mut current_durability = player.car.dur.real;
    while num_rolls_left > 0 {
        println!(
            "The cops are forming a {}! You have {} rolls left, Gambler!",
            "BLOCKADE".red(),
            num_rolls_left
        );
        let die1: u8 = get_random_number(6) as u8;
        let die2: u8 = get_random_number(6) as u8;
        let police_roll: u8 = get_random_number(6) as u8;
        println!(
            "You Rolled:\tDie 1\tDie 2\n\t\t\t {}[{}]\t {}[{}]",
            die1,
            die_from_u8(die1),
            die2,
            die_from_u8(die2)
        );

        println!("You can use one roll to go faster, and one to drive more defensively.");
        println!("Which roll do you want to apply to your SPEED? (1/2)");
        println!(
            "Die 1: [{}] {} + {} -> {} Total Speed",
            die1,
            die_from_u8(die1),
            player.car.spd.real,
            die1 as u32 + player.car.spd.real
        );
        println!(
            "Die 2: [{}] {} + {} -> {} Total Speed",
            die2,
            die_from_u8(die2),
            player.car.spd.real,
            die2 as u32 + player.car.spd.real
        );

        let other_die: u8;
        let die_choice = get_valid_input(&['1', '2']).unwrap().to_digit(10);
        match die_choice.unwrap() {
            1 => {
                distance_traveled += die1 as u32 + player.car.spd.real;
                other_die = die2;
                1
            }
            2 => {
                distance_traveled += die2 as u32 + player.car.spd.real;
                other_die = die1;
                2
            }
            _ => {
                panic!("BAD NUMBER!");
            }
        };

        println!("{}", progress_bar_head(distance_traveled, route_distance));

        println!(
            "Cops' Roll:\t\t\t[{}] {} + {} -> {} Total Attack",
            die_from_u8(police_roll),
            police_roll,
            heat,
            police_roll + heat as u8
        );
        println!(
            "Your Defense Roll: \t[{}] {} + {} -> {} Total Defense Score",
            die_from_u8(other_die),
            other_die,
            player.car.inc.real,
            other_die as u32 + player.car.inc.real
        );
        let total_heat: u32 = police_roll as u32 + heat;
        if other_die as u32 + player.car.inc.real < total_heat {
            println!("You've been rammed!");
            current_durability -= 1;
            if current_durability == 0 {
                return 0;
            }
        } else {
            println!("You might just make it yet!");
        }
        num_rolls_left -= 1;
        if num_rolls_left == 0 {
            if emergency_roll() < 10 {
                return -1;
            } else {
                println!("{}", "YOU MADE IT!".yellow());
                return current_durability as i32;
            }
        }
    }
    current_durability as i32
}

fn emergency_roll() -> i32 {
    println!("The cops have formed their blockade - you might be a goner! Roll those dice and pray, Gambler!");
    let die1: u8 = get_random_number(6) as u8;
    let die2: u8 = get_random_number(6) as u8;
    println!(
        "You Rolled a {}[{}] and a {}[{}]...",
        die1,
        die_from_u8(die1),
        die2,
        die_from_u8(die2)
    );
    return (die1 + die2) as i32;
}

fn choose_route() -> Route {
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
    let mut drawn_routes: Vec<usize> = Vec::new();

    // get 3 unique routes from the list of 6
    while drawn_routes.len() < 3 {
        let i: usize = get_random_number((routes.len() - 1) as i32) as usize;
        if !drawn_routes.contains(&i) {
            drawn_routes.push(i as usize);
        }
    }
    println!("Available Routes:\n");
    let mut i = 0;
    for route_index in &drawn_routes {
        i = i + 1;
        println!("Route #{}:", i.to_string().green());
        println!("{}\n", routes[*route_index]);
    }
    // Give route options
    println!("Pick a route (1/2/3):");
    // get input
    let routenum = get_valid_input(&['1', '2', '3']).unwrap().to_digit(10);
    let routenum = routenum.unwrap() - 1;

    Route {
        name: routes[drawn_routes[routenum as usize]].name.to_string(),
        distance: routes[drawn_routes[routenum as usize]].distance,
        heat: routes[drawn_routes[routenum as usize]].heat,
        prefereces: routes[drawn_routes[routenum as usize]].prefereces.clone(),
        prices: routes[drawn_routes[routenum as usize]].prices.clone(),
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
// TIER 4  ////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////

fn get_random_number(_d: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(0.._d) + 1;
    return random_number;
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

fn wait_for_enter() {
    loop {
        if let Ok(event::Event::Key(KeyEvent {
            code: KeyCode::Enter,
            ..
        })) = event::read()
        {
            break;
        }
    }
}

fn prompt_to_continue(string: Option<String>) {
    match string {
        Some(contents) => match contents.as_str() {
            "roll" => println!("Press ENTER/RETURN to ROLL!"),
            _ => println!(
                "Press ENTER/RETURN to CONTINUE to {}",
                contents.to_uppercase()
            ),
        },
        None => println!("Press ENTER/RETURN to CONTINUE..."),
    }
    wait_for_enter();
}

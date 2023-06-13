use crossterm::event::{read, Event, KeyCode, KeyEventKind, KeyEvent};
use crossterm::style::{SetForegroundColor, Stylize};
//use crossterm::terminal::{Clear, ClearType};
//use crossterm::{execute, Result};
use lib::{car::Car, player::Player, route::Route, stat::Stat, sutil::*};
use rand::Rng;
use std::{io::Write, thread, time::Duration, write};

pub mod lib;

pub static MAX_STAT: u32 = 24;
pub static CAR_STAT_LENGTH: u8 = 12;
pub static MONEY_MULT: f64 = 10.0;

////////////////////////////////////////////////////////////////////////////////////////
// MAIN ////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    //crossterm::terminal::enable_raw_mode();
    clear();
    let mut player: Player = start();
    let mut player_quit = false;

    while !player_quit {
        print_solo("Welcome to Moonshine Runner!".to_string());
        print_bottles();
        prompt_to_continue(None);
        let mut end_round = false;
        while !end_round {
            // BREW
            print_header(&mut player, 1);
            //print_barn();
            prompt_to_continue(Some("brew stage".to_string()));
            brew(&mut player);

            // DRIVE
            prompt_to_continue(Some("drive stage".to_string()));
            let (cargo_status, route) = drive(&mut player);

            //BARTER
            if cargo_status > 0 {
                clear();
                prompt_to_continue(Some("barter stage".to_string()));
                barter(&mut player, cargo_status, route);
            } else if cargo_status < 0 {
                player.car = default_car();
                // player is boned
                break;
            }

            // BUY
            prompt_to_continue(Some("buy stage".to_string()));
            buy(&mut player);

            if player.money >= 10000 {
                println!("You won!");
                end_round = true;
                player_quit = true;
            }
        }
    }
    quit();
}

///////////////////////////////////////////////////////////////////////////////////////////
// TIER 2  ////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////

fn start() -> Player {
    let mut player = Player::new();
    player.money = 1000;
    player.car.name = "[F] Rusty Hatchback".to_string();
    player.car.spd = Stat::new(2, 5);
    player.car.dur = Stat::new(2, 5);
    player.car.cgo = Stat::new(8, 15);
    player.car.inc = Stat::new(2, 5);
    player.car.flavor = "Have you had your tetanus shots?".to_string();
    player
}

fn default_car() -> Car {
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

fn brew(mut player: &mut Player) {
    print_header(player, 1);
    //print_barn();
    //prompt_to_continue(Some("brew stage".to_string()));
    print_header(player, 1);
    print_brew_stage(player);
    println!();
    for i in 0..36 {
        print!("\r[");
        for j in 0..i {
            print!("=")
        }
        for j in i..36 - 1 {
            print!(" ");
        }
        print!("]");

        // Flush the output to make it visible immediately
        std::io::stdout().flush().unwrap();

        // Wait for a short duration

        thread::sleep(Duration::from_millis(30));
    }
    println!();
    println!();
    //take into account player's still size and quality
    let quality_odds_map = match player.still.qlt.real {
        12 => (5, 10, 85),
        11 => (10, 15, 75),
        10 => (15, 20, 65),
        9 => (20, 25, 55),
        8 => (25, 30, 45),
        7 => (30, 35, 35),
        6 => (35, 40, 25),
        5 => (40, 45, 15),
        4 => (45, 45, 10),
        3 => (50, 40, 10),
        2 => (60, 35, 5),
        1 => (70, 25, 5),
        0 => (80, 15, 5),
        _ => panic!("Invalid stat value!"),
    };
    let roll = get_random_number(100);
    let quality = match roll {
        v if v < quality_odds_map.0 => 1,
        v if v < quality_odds_map.0 + quality_odds_map.1 => 2,
        _ => 3,
    };
    // Set Quantity
    let output = (player.still.spd.real * player.still.vol.real) * 2;
    player.car.cargo_quantity = calculate_cargo_quantity(output, player.car.cgo.real);
    match quality {
        1 => {
            println!(
                "Yeesh! You brewed {} cases of '{}' Good luck getting rid of this!",
                output,
                "Rotgut Wiskee".red()
            );
            player.car.cargo_quality = 1;
        }
        2 => {
            println!(
                "You brewed {} cases of '{}.' This will be plenty easy to move!",
                output,
                "OK Hooch".yellow()
            );
            player.car.cargo_quality = 2;
        }
        3 => {
            println!(
                "Hoo wee! This here's {} cases of '{}!' Worth top dollar, to the right buyer...",
                output,
                "White Lightning".green()
            );
            player.car.cargo_quality = 1;
        }
        _ => panic!("{}", "BAD NUMBER!".red()),
    }
}

fn drive(mut player: &mut Player) -> (i32, Route) {
    print_header(player, 2);
    let booze = match player.car.cargo_quality {
        1 => "Rotgut Wiskee".red(),
        2 => "OK Hooch".yellow(),
        3 => "White Lightning".green(),
        _ => panic!("Invalid stat value!"),
    };
    println!(
        "| You fit {} cases of {} into your {}.\n| (total capacity: {}).\n",
        player.car.cargo_quantity.to_string().cyan(),
        booze,
        player.car.name.to_string().green(),
        player.car.cgo.real
    );
    let route = choose_route();
    let route_distance = route.distance;
    let heat = route.heat;

    let mut distance_traveled: u32 = 0;
    let player_still_alive = true;
    let mut player_spotted = false;

    // Set Player Car Durability
    player.car.current_durability = player.car.dur.real;

    print_header(player, 2);
    println!("| Now you're in the {} stage.", "DRIVE".green());
    println!("| {}", "Start your engine, gambler!".red());
    prompt_to_continue(None);
    while distance_traveled < route_distance && player_still_alive {
        // Normalize distance_traveled
        if distance_traveled > route_distance {
            distance_traveled = route_distance;
        }
        // Print Header
        print_header(player, 2);

        // Do Checks
        if player_spotted {
            if distance_traveled >= route_distance {
                println!("Ha! You squeaked right by 'em!");
                prompt_to_continue(None);
                return ((player.car.dur.real as i32 + 1), route);
            }
            let chase_result = chase(&mut player, route, distance_traveled, heat);
            match chase_result {
                (-1, _) => println!("Your whiskey and your ride are history, Gambler!"),
                (0, _) => print_solo_bad(
                    "You've been wrecked! Back to the moonshine still...".to_string(),
                ),
                _ => (),
            }
            player.car.current_durability = player.car.dur.real;
            prompt_to_continue(None);
            return chase_result;
        }

        // Part 1 - Start of Round Stage
        print_drive_info(player, distance_traveled, route_distance, &route.name);
        //prompt_to_continue(Some("roll".to_string()));
        let die1: u8 = get_random_number(6) as u8 + 1;
        let die2: u8 = get_random_number(6) as u8 + 1;
        let police_roll: u8 = get_random_number(6) as u8 + 1;

        // Part 2 - Roll Dice
        print_roll(die1, die2);
        // Have Player Choose Die
        print_roll_prompt(player, true, die1, die2);
        // Process Consequences of Die Choice
        let other_die: u8;
        let die_choice = get_instant_input(&['1', '2']).unwrap().to_digit(10);
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

        // Part 3 - Consequences, John.
        print_header(player, 2);
        print_roll(die1, die2);
        print_separator();
        println!("| {}", "STEALTH CHECK:".blue().italic());
        println!(
            "| {:<20}[{}] {} + {} -> {} Total Incognito Score",
            "Your Stealth Roll:".green(),
            die_from_u8(other_die),
            other_die,
            player.car.inc.real,
            other_die as u32 + player.car.inc.real
        );
        println!(
            "| {:<20}[{}] {} + {} Route Heat -> {} Total Heat",
            "Cops' Roll:".red(),
            die_from_u8(police_roll),
            police_roll,
            heat,
            police_roll + heat as u8
        );
        // Calculate Heat
        let total_heat: u32 = police_roll as u32 + heat;
        // Check Roll
        if other_die as u32 + player.car.inc.real < total_heat && distance_traveled < route_distance
        {
            println!();
            println!("\t{}", "You've been made! Floor it!".red());
            player_spotted = true;
        } else if distance_traveled < route_distance {
            println!(
                "\n| {} ",
                "Good work, Gambler. They haven't spotted you yet!".cyan()
            );
        }
        prompt_to_continue(None);
    }
    print_solo("You made it!".to_string());
    prompt_to_continue(None);
    return ((player.car.dur.real as i32 + 1), route);
}

fn barter(mut player: &mut Player, cargo_status: i32, route: Route) {
    let mut mult: f64 = 1.0;
    print_header(player, 3);
    prompt_to_continue(Some("roll".to_string()));
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
    mult = mult * cargo_status as f64 * player.car.cargo_quality as f64 * MONEY_MULT;
    mult = mult * route.prices[player.car.cargo_quality as usize] as f64;
    let die = get_random_number(6) + 1;
    let money_increment = match die {
        1 => mult * 0.65,
        2 => mult * 0.75,
        3 => mult * 0.85,
        4 => mult * 0.95,
        5 => mult * 1.0,
        6 => mult * 1.15,
        _ => panic!("The die has died and turned into nonsense. Seek professional help."),
    };
    println!(
        "You rolled a {}! Given this and the condition of your cargo, you get: ${:.2}",
        die.to_string().yellow().bold(),
        (money_increment as i32).to_string().green().bold()
    );
    prompt_to_continue(None);
    player.money += money_increment as i32;
}

fn buy(player: &mut Player) {
    let mut user_entry: Option<char> = None;
    while user_entry != Some('\0') {
        print_header(player, 4);
        println!("Visit the Auction House to buy a new car (A), Upgrade Your Still (S), Buy Car Upgrades (C) or Continue (TAB)");
        user_entry = get_instant_input(&['a', 's', 'c', '\0']);
        let success: bool = match user_entry {
            Some('a') => { auction_house(player); true},
            Some('s') => shop_category(player, 's'),
            Some('c') => shop_category(player, 'c'),
            _ => false
        };
        if success {
            print_header(player, 4);
            println!("Congratulations on your snazzy new upgrade.\n");
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
// TIER 3  ////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////

fn auction_house(mut player: &mut Player) {
    print_header(player, -1);
    println!(
        "| Your Car: {}\n| Trade-In Value: {}",
        player.car.to_string(),
        (player.car.price as f64 * 0.75).round() as i32
    );
    let car1: Car = Car::generate(1);
    let car2: Car = Car::generate(2);
    let car3: Car = Car::generate(3);
    println!(
        "| Available Cars: \n| {}\n| ${}\n| {}\n| ${}\n| {}\n| ${}",
        car1.to_string(),
        car1.price.to_string(),
        car2.to_string(),
        car2.price.to_string(),
        car3.to_string(),
        car3.price.to_string(),
    );
    println!("Select a car you'd like to buy (1/2/3)");
    match get_instant_input(&['1', '2', '3']).unwrap() {
        '1' => {
            if player.money >= car1.price as i32 {
                player.money -=
                    (car1.price as i32 - (player.car.price as f64 * 0.75).round() as i32);
                player.car = car1;
            } else {
                println!("You don't have enough money to buy this car.");
            }
        }
        '2' => {
            if player.money >= car2.price as i32 {
                player.money -=
                    (car2.price as i32 - (player.car.price as f64 * 0.75).round() as i32);
                player.car = car2;
            } else {
                println!("You don't have enough money to buy this car.");
            }
        }
        '3' => {
            if player.money >= car3.price as i32 {
                player.money -=
                    (car3.price as i32 - (player.car.price as f64 * 0.75).round() as i32);
                player.car = car3;
            } else {
                println!("You don't have enough money to buy this car.");
            }
        }
        _ => {
            panic!("Invalid Input")
        }
    }
}

fn shop_category(player: &mut Player, category_code: char) -> bool {
    print_header(player, 4);
    let print_msg: String = format!("Your Money: {}", player.money);
    println!("{}", print_msg.green());
    match category_code {
        'c' => {
            println!("OK, let's look at some car upgrades.");
            println!(
                "Do you want to upgrade:\nSPEED (${}), DURABILITY (${}) or CARGO (${})? (s/d/c)",
                (pow(1.16, player.car.spd.real as f64) * 100.0) as i32,
                (pow(1.16, player.car.dur.real as f64) * 100.0) as i32,
                (pow(1.16, player.car.cgo.real as f64) * 100.0) as i32
            );
            return try_buy(
                player,
                category_code,
                get_instant_input(&['s', 'd', 'c']).unwrap(),
            );
        }
        's' => {
            println!("OK, let's look at some still upgrades.");
            println!(
                "Do you want to upgrade:\nVOLUME (${}), SPEED (${}) or QUALITY (${})? (v/s/q)",
                (pow(1.16, player.still.vol.real as f64) * 100.0) as i32,
                (pow(1.16, player.still.spd.real as f64) * 100.0) as i32,
                (pow(1.16, player.still.qlt.real as f64) * 100.0) as i32
            );
            return try_buy(
                player,
                category_code,
                get_instant_input(&['v', 's', 'q']).unwrap(),
            );
        }
        _ => {
            return false;
        }
    }
}

fn try_buy(mut player: &mut Player, category_code: char, item_code: char) -> bool {
    match category_code {
        // Car Upgrades
        'c' => match item_code {
            's' => {
                if player.car.spd.real < player.car.spd.max {
                    let cost = (player.car.spd.real * 10) as i32;
                    if player.money >= cost {
                        player.car.spd.real += 1;
                        player.money -= cost;
                        return true;
                    } else {
                        println!("Not enough change, chump!");
                    }
                } else {
                    println!("Woah there! You're already maxed out, speed racer!");
                }
                return false;
            }
            'd' => {
                if player.car.dur.real < player.car.dur.max {
                    let cost = (player.car.dur.real * 10) as i32;
                    if player.money >= cost {
                        player.car.dur.real += 1;
                        player.money -= cost;
                        return true;
                    } else {
                        println!("Not enough change, chump!");
                    }
                } else {
                    println!("You're already maxed out, speed racer!");
                }
                return false;
            }
            'c' => {
                if player.car.cgo.real < player.car.cgo.max {
                    let cost = (player.car.cgo.real * 10) as i32;
                    if player.money >= cost {
                        player.car.cgo.real += 1;
                        player.money -= cost;
                        return true;
                    } else {
                        println!("Not enough change, chump!");
                    }
                } else {
                    println!("You're already maxed out, speed racer!");
                }
                return false;
            }
            c => {
                panic!("How did you get {}?", c);
            }
            _ => {
                panic!("This isn't supposed to happen.");
            }
        },
        // Still Upgrades
        's' => match item_code {
            's' => {
                if player.still.spd.real < player.still.spd.max {
                    let cost = (player.still.spd.real * 10) as i32;
                    if player.money >= cost {
                        player.still.spd.real += 1;
                        player.money -= cost;
                        return true;
                    } else {
                        println!("Not enough change, chump!");
                    }
                } else {
                    println!("Woah there Pappy, you're already maxed out!");
                }
                return false;
            }
            'v' => {
                if player.still.vol.real < player.still.vol.max {
                    let cost = (player.still.vol.real * 10) as i32;
                    if player.money >= cost {
                        player.still.vol.real += 1;
                        player.money -= cost;
                        return true;
                    } else {
                        println!("Not enough change, chump!");
                    }
                } else {
                    println!("Woah there Pappy, you're already maxed out!");
                }
                return false;
            }
            'q' => {
                if player.still.qlt.real < player.still.qlt.max {
                    let cost = (player.still.qlt.real * 10) as i32;
                    if player.money >= cost {
                        player.still.qlt.real += 1;
                        player.money -= cost;
                        return true;
                    } else {
                        println!("Not enough change, chump!");
                    }
                } else {
                    println!("Woah there Pappy, you're already maxed out!");
                }
                return false;
            }
            c => {
                panic!("How did you get {}?", c);
            }
            _ => {
                panic!("This isn't supposed to happen.");
            }
        },
        _ => {
            panic!("try_buy(): Invalid Category Code!");
        }
    }
}

fn chase(
    mut player: &mut Player,
    route: Route,
    mut distance_traveled: u32,
    heat: u32,
) -> (i32, Route) {
    SetForegroundColor(crossterm::style::Color::Red);
    // Initialize Chase & Calculate number of Rolls till Blockade
    let route_distance = route.distance;
    let mut num_rolls_left = 1 + (route_distance - distance_traveled) / (player.car.spd.real + 3); // was divided by speed + 4

    // Start Loop
    while num_rolls_left > 0 {
        // Header n Stuff
        print_header(player, 2);
        print_drive_info(player, distance_traveled, route_distance, &route.name);

        // Check if the player has arrived
        if distance_traveled >= route_distance {
            return ((player.car.current_durability as i32), route);
        }

        // Blockade Warning
        println!(
            "| {} The cops are forming a {} in {} rolls!",
            "STATUS: ".cyan(),
            "BLOCKADE".red(),
            num_rolls_left
        );
        print_separator();

        // Roll Dice & Prompt for player input
        // Here's where the player would roll the dice if that animation existed
        let die1: u8 = get_random_number(6) as u8 + 1;
        let die2: u8 = get_random_number(6) as u8 + 1;
        let police_roll: u8 = get_random_number(6) as u8 + 1;
        print_roll(die1, die2);
        print_roll_prompt(player, false, die1, die2);
        // Get & Process the player input
        let other_die: u8;
        let die_choice = get_instant_input(&['1', '2']).unwrap().to_digit(10);
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

        // Consequences...

        // Same Header Order
        // Feel like I shouldn't print car here, maybe. Seems distracting.
        print_header(player, 2);
        print_drive_info(player, distance_traveled, route_distance, &route.name);
        println!(
            "| {} The cops are forming a {} in {} rolls!",
            "STATUS: ".cyan(),
            "BLOCKADE".red(),
            num_rolls_left
        );
        print_separator();

        
        print_roll(die1, die2); // Should CROSS OUT die that's been used
                                // Give Info
        print_separator();
        println!("| {}", "DEFENSE CHECK:".blue().italic());
        println!(
            "| {:<20}[{}] {} + {} Route Heat -> {} Total Attack",
            "Cops' Attack Roll:".red(),
            die_from_u8(police_roll),
            police_roll,
            heat,
            ((police_roll + heat as u8).to_string()).red().bold()
        );
        println!(
            "| {:<20}[{}] {} + {} -> {} Total Defense",
            "Your Defense Roll:".green(),
            die_from_u8(other_die),
            other_die,
            player.car.inc.real,
            ((other_die as u32 + player.car.inc.real).to_string())
                .green()
                .bold()
        );

        println!();

        // Give Results
        let total_heat: u32 = police_roll as u32 + heat;
        if other_die as u32 + player.car.inc.real < total_heat {
            println!(
                "| Cops' Attack exceeds your Defense:\n| {}",
                "You've been rammed!".red()
            );
            player.car.current_durability -= 1;
            if player.car.current_durability == 0 {
                return (0, route);
            }
        } else if distance_traveled < route_distance {
            println!("| {}", "You might just make it yet!".green());
        } else {
            println!("\n| {}! You made it by the seat of your pants!", "Holy Hockey Sticks".bold().green());
        }
        num_rolls_left -= 1;
        if num_rolls_left == 0 && distance_traveled < route_distance {
            if emergency_roll() < 10 {
                return (-1, route);
            } else {
                println!("{}", "YOU MADE IT!!".bold().green());
                return ((player.car.current_durability as i32), route);
            }
        }
        prompt_to_continue(None);
        clear();
    }
    ((player.car.current_durability as i32), route)
}

fn emergency_roll() -> i32 {
    println!("The cops have formed their blockade - you might be a goner! Roll those dice and pray, Gambler!\nRemember: you need to roll a {} or you're {}.", "10".bold().red(), "BUSTED".bold().red());
    let die1: u8 = get_random_number(6) as u8 + 1;
    let die2: u8 = get_random_number(6) as u8 + 1;
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
        // Route 1 - Middle of Nowhere
        Route {
            name: String::from("Route 4"),
            distance: 24,
            heat: 0,
            prefereces: vec![4, 3, 2],
            prices: vec![3, 2, 2],
        },
        // Route 2 - Middle of Nowhere too
        Route {
            name: String::from("Country Road 97"),
            distance: 64,
            heat: 1,
            prefereces: vec![4, 3, 2],
            prices: vec![4, 4, 3],
        },
        // Route 3 - The Docks
        Route {
            name: String::from("Marina Dr."),
            distance: 32,
            heat: 3,
            prefereces: vec![4, 3, 2],
            prices: vec![5, 7, 4],
        },
        // Route 4 - Some Quiet Cabins
        Route {
            name: String::from("Lakeside Ave"),
            distance: 48,
            heat: 5,
            prefereces: vec![4, 3, 2],
            prices: vec![2, 8, 6],
        },
        // Route 5
        Route {
            name: String::from("Main St. Downtown"),
            distance: 24,
            heat: 8,
            prefereces: vec![4, 3, 2],
            prices: vec![3, 10, 8],
        },
        // Route 6
        Route {
            name: String::from("Main St. Uptown"),
            distance: 32,
            heat: 10,
            prefereces: vec![1, 3, 2],
            prices: vec![1, 10, 16],
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
    println!("| Choose from the available routes. If a route's heat is higher than \n| your car's Incognito Score (INC: {}), it'll be risky!\n", "INSERT_CAR_HERE");
    println!("\tAvailable Routes:\n");
    let mut routes_fields: Vec<Vec<String>> = Vec::new();

    let mut i = 0;
    for route_index in &drawn_routes {
        let mut result: Vec<String> = Vec::new();
        i = i + 1;
        result.push(format!("Route #{}:            ", i.to_string().green()));
        for field in routes.get(*route_index).unwrap().clone().get_all_fields() {
            result.push(field.to_string());
        }
        routes_fields.push(result);
    }

    for line_num in 0..9 {
        for column in 0..3 {
            print!(
                "| {:<21} ",
                routes_fields.get(column).unwrap().get(line_num).unwrap()
            );
        }
        print!(" |");
        println!();
    }
    // Give route options
    println!("\n\t{}", "Pick a route (1/2/3):\n".cyan().bold());
    // get input
    let routenum = get_instant_input(&['1', '2', '3']).unwrap().to_digit(10);
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

fn pow(base: f64, exp: f64) -> f64 {
    let mut result = 1.0;
    for _ in 0..exp as i32 {
        result *= base;
    }
    result
}

fn calculate_cargo_quantity(still_capacity: u32, car_capacity: u32) -> u32 {
    if car_capacity > still_capacity {
        still_capacity
    } else {
        car_capacity
    }
}

fn get_random_number(_d: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(0.._d);
    return random_number;
}

fn get_instant_input(chars: &[char]) -> Option<char> {
    let input_is_valid: bool = false;
    while !input_is_valid {
        if let Some(input) = await_key_down() {
            if chars.contains(&input) {
                return Some(input);
            } else {
                println!("Invald input");
            }
        }
    }
    None
}

fn wait_for_enter() {
    crossterm::terminal::enable_raw_mode();
    let mut player_pressed_enter = false;
    while !player_pressed_enter {
        if let Ok(Event::Key(key_event)) = read() {
            match key_event.code {
                KeyCode::Enter => player_pressed_enter = true,
                KeyCode::Esc => {
                    if crossterm::terminal::is_raw_mode_enabled().unwrap() {
                        crossterm::terminal::disable_raw_mode();
                    }
                    quit();
                }
                _ => continue,
            }
        }
    }
    crossterm::terminal::disable_raw_mode();
}

fn await_key_down() -> Option<char> {
    let mut key: Option<char> = None;
    let mut key_down: Option<crossterm::event::KeyCode> = None;

    //let mut prev_key_down: Option<crossterm::event::KeyCode> = None; // dont need
    
    while key_down == None {
        crossterm::terminal::enable_raw_mode();
        if let Event::Key(event) = read().ok()? {
            // MATCH EVENT::KEYEVENTKIND FOR "PRESS" TYPE
            key_down = match event.kind {
                KeyEventKind::Press => Some(event.code),
                _ => None,
            }
        }
    }

    key = match key_down {
        Some(KeyCode::Char(c)) => Some(c),
        Some(KeyCode::Tab) => Some('\0'),
        Some(KeyCode::Esc) => {
            quit();
            None
        },
        _ => None,
    };

    if crossterm::terminal::is_raw_mode_enabled().unwrap() {
        crossterm::terminal::disable_raw_mode();
    }

    key
}

fn prompt_to_continue(string: Option<String>) {
    println!();
    match string {
        Some(contents) => match contents.as_str() {
            "roll" => println!(
                "\tPress {} to {}!",
                "ENTER/RETURN".bold().cyan(),
                "ROLL".bold().cyan()
            ),
            _ => println!(
                "\tPress {} to CONTINUE to {}",
                "ENTER/RETURN".bold().cyan(),
                contents.to_uppercase().bold().cyan()
            ),
        },
        None => println!(
            "\tPress {} to {}...",
            "ENTER/RETURN".bold().cyan(),
            "CONTINUE".bold().cyan()
        ),
    }
    println!();

    wait_for_enter();
}

fn quit() {
    if crossterm::terminal::is_raw_mode_enabled().unwrap() {
        crossterm::terminal::disable_raw_mode();
    }
    println!("Thanks for playing!");
    std::process::exit(0);
}

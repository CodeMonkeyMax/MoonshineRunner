use crossterm::event::{read, Event, KeyCode};
use crossterm::style::{SetForegroundColor, Stylize};
use lib::still::Still;
use lib::wallet::Wallet;
//use crossterm::terminal::{Clear, ClearType};
//use crossterm::{execute, Result};
use lib::{car::Car, player::Player, route::Route, stat::Stat, sutil::*};
use rand::Rng;
use std::{io::Write, thread, time::Duration, write};

pub mod lib;

pub static MAX_STAT: u32 = 24;
pub static CAR_STAT_LENGTH: u8 = 12;
pub static MONEY_MULT: f64 = 10.0;

static PLAYER: Player = Player {
    wallet: Wallet { money: 1000 },
    car: Car::default_car(),
    still: Still::default(),
};

////////////////////////////////////////////////////////////////////////////////////////
// MAIN ////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    //crossterm::terminal::enable_raw_mode();
    clear();
    let mut player_quit = false;

    while !player_quit {
        print_solo("Welcome to Moonshine Runner!".to_string());
        print_bottles();
        prompt_to_continue(None);
        let mut end_round = false;
        while !end_round {
            // BREW
            print_header(1);
            print_barn();
            prompt_to_continue(Some("brew stage".to_string()));
            brew();

            // DRIVE
            prompt_to_continue(Some("drive stage".to_string()));
            let (cargo_status, route) = drive();

            //BARTER
            if cargo_status > 0 {
                clear();
                prompt_to_continue(Some("barter stage".to_string()));
                barter(cargo_status, route);
            } else if cargo_status < 0 {
                unsafe {
                    PLAYER.car.reset();
                }

                // player is boned
                break;
            }

            // BUY
            prompt_to_continue(Some("buy stage".to_string()));
            buy();

            if PLAYER.get_money() >= 100000 {
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

fn brew() {
    print_header(1);
    //print_barn();
    //prompt_to_continue(Some("brew stage".to_string()));
    print_header(1);
    print_brew_stage();
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

        thread::sleep(Duration::from_millis(50));
    }
    println!();
    println!();
    unsafe {
        //take into account player's still size and quality
        let still: Still = *PLAYER.get_still();
        let quality_odds_map = match still.qlt.real {
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
        let output = (PLAYER.get_still().spd.real * PLAYER.still.vol.real) * 2;
        PLAYER.car.cargo_quantity = calculate_cargo_quantity(output, PLAYER.car.cgo.real);
        match quality {
            1 => {
                println!(
                    "Yeesh! You brewed {} cases of '{}' Good luck getting rid of this!",
                    output,
                    "Rotgut Wiskee".red()
                );
                PLAYER.car.cargo_quality = 1;
            }
            2 => {
                println!(
                    "You brewed {} cases of '{}.' This will be plenty easy to move!",
                    output,
                    "OK Hooch".yellow()
                );
                PLAYER.car.cargo_quality = 2;
            }
            3 => {
                println!(
                "Hoo wee! This here's {} cases of '{}!' Worth top dollar, to the right buyer...",
                output,
                "White Lightning".green()
            );
                PLAYER.car.cargo_quality = 3;
            }
            _ => panic!("{}", "BAD NUMBER!".red()),
        }
    }
}

fn drive() -> (i32, Route) {
    print_header(2);
    let car = PLAYER.get_car();
    let booze = match car.cargo_quality {
        1 => "Rotgut Wiskee".red(),
        2 => "OK Hooch".yellow(),
        3 => "White Lightning".green(),
        _ => panic!("Invalid stat value!"),
    };
    println!(
        "| You fit {} cases of {} into your {}.\n| (total capacity: {}).\n",
        PLAYER.get_car().cargo_quantity.to_string().cyan(),
        booze,
        PLAYER.get_car().name.to_string().green(),
        PLAYER.get_car().cgo.real
    );
    let route = choose_route();
    let route_distance = route.distance;
    let heat = route.heat;

    let mut distance_traveled: u32 = 0;
    let player_still_alive = true;
    let mut player_spotted = false;

    unsafe {
        // Set Player Car Durability
        PLAYER.car.current_durability = PLAYER.car.dur.real;
    }

    print_header(2);
    println!("| Now you're in the {} stage.", "DRIVE".green());
    println!("| {}", "Start your engine, gambler!".red());
    prompt_to_continue(None);
    while distance_traveled < route_distance && player_still_alive {
        // Initialize the round
        if distance_traveled > route_distance {
            distance_traveled = route_distance;
        }
        // Print Header
        print_header(2);
        // Check if the player has been spotted
        if player_spotted {
            if distance_traveled == route_distance {
                println!("Ha! You squeaked right by 'em!");
                return ((PLAYER.get_car().dur.real as i32 + 1), route);
            }
            let chase_result = chase(route, distance_traveled, heat);
            match chase_result {
                (-1, _) => println!("Your whiskey and your ride are history, Gambler!"),
                (0, _) => print_solo_bad(
                    "You've been wrecked! Back to the moonshine still...".to_string(),
                ),
                (_, _) => println!("Holy Hockey Sticks! You made it by the seat of your pants!"),
            }
            PLAYER.get_car().current_durability = PLAYER.get_car().dur.real;
            return chase_result;
        }

        // Part 1 - Start of Round Stage
        print_drive_stage(distance_traveled, route_distance, &route.name);
        //prompt_to_continue(Some("roll".to_string()));
        let die1: u8 = get_random_number(6) as u8;
        let die2: u8 = get_random_number(6) as u8;
        let police_roll: u8 = get_random_number(6) as u8;

        // Part 2 - Roll Dice
        print_header(2);
        if distance_traveled >= 1 {
            println!(
                "| {} Good work, Gambler. They haven't spotted you yet!",
                "STATUS: ".cyan()
            );
            print_separator();
        }
        print_drive_stage(distance_traveled, route_distance, &route.name);

        print_roll(die1, die2);
        // Have Player Choose Die
        print_roll_prompt(true, die1, die2);
        // Process Consequences of Die Choice
        let other_die: u8;
        let die_choice = get_instant_input(&['1', '2']).unwrap().to_digit(10);
        match die_choice.unwrap() {
            1 => {
                distance_traveled += die1 as u32 + PLAYER.get_car().spd.real;
                other_die = die2;
                1
            }
            2 => {
                distance_traveled += die2 as u32 + PLAYER.get_car().spd.real;
                other_die = die1;
                2
            }
            _ => {
                panic!("BAD NUMBER!");
            }
        };

        // Part 3 - Consequences, John.
        print_header(2);
        if distance_traveled >= 1 && !player_spotted {
            println!(
                "| {} Good work, Gambler. They haven't spotted you yet!",
                "STATUS: ".cyan()
            );
            print_separator();
        }

        println!(
            "| Your stealth roll: \t[{}] {} + {} -> {} total incognito score",
            die_from_u8(other_die),
            other_die,
            PLAYER.get_car().inc.real,
            other_die as u32 + PLAYER.get_car().inc.real
        );
        println!(
            "| Cops' Roll:\t\t\t[{}] {} + {} Route Heat -> {} Total Heat",
            die_from_u8(police_roll),
            police_roll,
            heat,
            police_roll + heat as u8
        );
        // Calculate Heat
        let total_heat: u32 = police_roll as u32 + heat;
        // Check Roll
        if other_die as u32 + PLAYER.get_car().inc.real < total_heat
            && distance_traveled < route_distance
        {
            println!();
            println!("{}", "You've been made! Floor it!".red());
            player_spotted = true;
        }
        prompt_to_continue(None);
    }
    print_solo("You made it!".to_string());
    prompt_to_continue(None);
    return ((PLAYER.get_car().dur.real as i32 + 1), route);
}

fn barter(cargo_status: i32, route: Route) {
    let mut mult: f64 = 1.0;
    print_header(3);
    match cargo_status {
        -1 => panic!("Player has negative cargo status - should not be in 'barter()' at all!"),
        0 => {
            mult = 0.5;
        }
        _ => {
            mult = (cargo_status as f64 / (2.0 * PLAYER.get_car().dur.real as f64)) + 0.5;
        }
    }
    println!("DEBUG: Mult = {}", mult);
    mult = mult * cargo_status as f64 * PLAYER.get_car().cargo_quality as f64 * MONEY_MULT;
    mult = mult * route.prices[PLAYER.get_car().cargo_quality as usize] as f64;
    let die = get_random_number(6);
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
        "You rolled a {}. Given this and the results of your drive, you get: ${:.2}",
        die.to_string().yellow().bold(),
        (money_increment as i32).to_string().green().bold()
    );
    PLAYER.wallet.add(money_increment as i32);
}

fn buy() {
    print_header(4);
    println!("Visit the Auction House to buy new car? (y/n)");
    let mut user_entry: Option<char> = get_instant_input(&['y', 'n']);
    if user_entry == Some('y') {
        auction_house();
    }
    print_header(4);
    println!("Do you want to buy any upgrades? (y/n)");
    let mut user_entry: Option<char> = get_instant_input(&['y', 'n']);
    while user_entry != Some('n') {
        print_header(4);
        println!("Do you want to buy upgrades for your CAR or for your STILL? (c/s)");
        if shop_category(get_instant_input(&['c', 's']).unwrap()) {
            print_header(4);
            println!("Congratulations on your snazzy new upgrade.\n");
        }
        println!("{}", "Shop Again? (y/n)\n".cyan().bold());
        user_entry = get_instant_input(&['y', 'n']);
    }
    if user_entry == Some('y') {}
}

///////////////////////////////////////////////////////////////////////////////////////////
// TIER 3  ////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////

fn auction_house() {
    print_header(-1);
    println!(
        "| Your Car: {}\n| Trade-In Value: {}",
        PLAYER.get_car().to_string(),
        (PLAYER.get_car().price as f64 * 0.75).round() as i32
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
    unsafe {
        println!("Select a car you'd like to buy (1/2/3)");
        match get_instant_input(&['1', '2', '3']).unwrap() {
            '1' => {
                if PLAYER.get_money() >= car1.price as i32 {
                    PLAYER.wallet.sub(
                        car1.price as i32 - (PLAYER.get_car().price as f64 * 0.75).round() as i32,
                    );
                    PLAYER.car = car1;
                } else {
                    println!("You don't have enough money to buy this car.");
                }
            }
            '2' => {
                if PLAYER.get_money() >= car2.price as i32 {
                    PLAYER.wallet.sub(
                        car2.price as i32 - (PLAYER.get_car().price as f64 * 0.75).round() as i32,
                    );
                    PLAYER.car = car2;
                } else {
                    println!("You don't have enough money to buy this car.");
                }
            }
            '3' => {
                if PLAYER.get_money() >= car3.price as i32 {
                    PLAYER.wallet.sub(
                        car3.price as i32 - (PLAYER.get_car().price as f64 * 0.75).round() as i32,
                    );
                    PLAYER.car = car3;
                } else {
                    println!("You don't have enough money to buy this car.");
                }
            }
            _ => {
                panic!("Invalid Input")
            }
        }
    }
}

fn shop_category(category_code: char) -> bool {
    print_header(4);
    let print_msg: String = format!("Your Money: {}", PLAYER.get_money());
    println!("{}", print_msg.green());
    match category_code {
        'c' => {
            println!("OK, let's look at some car upgrades.");
            println!(
                "Do you want to upgrade:\nSPEED (${}), DURABILITY (${}) or CARGO (${})? (s/d/c)",
                (pow(1.16, PLAYER.get_car().spd.real as f64) * 100.0) as i32,
                (pow(1.16, PLAYER.get_car().dur.real as f64) * 100.0) as i32,
                (pow(1.16, PLAYER.get_car().cgo.real as f64) * 100.0) as i32
            );
            return try_buy(category_code, get_instant_input(&['s', 'd', 'c']).unwrap());
        }
        's' => {
            println!("OK, let's look at some still upgrades.");
            println!(
                "Do you want to upgrade:\nVOLUME (${}), SPEED (${}) or QUALITY (${})? (v/s/q)",
                (pow(1.16, PLAYER.get_still().vol.real as f64) * 100.0) as i32,
                (pow(1.16, PLAYER.get_still().spd.real as f64) * 100.0) as i32,
                (pow(1.16, PLAYER.get_still().qlt.real as f64) * 100.0) as i32
            );
            return try_buy(category_code, get_instant_input(&['v', 's', 'q']).unwrap());
        }
        _ => {
            return false;
        }
    }
}

fn try_buy(category_code: char, item_code: char) -> bool {
    match category_code {
        // Car Upgrades
        'c' => match item_code {
            's' => {
                if PLAYER.get_car().spd.real < PLAYER.get_car().spd.max {
                    let cost = (PLAYER.get_car().spd.real * 10) as i32;
                    if PLAYER.get_money() >= cost {
                        PLAYER.car.spd.real += 1;
                        PLAYER.wallet.sub(cost);
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
                if PLAYER.get_car().dur.real < PLAYER.get_car().dur.max {
                    let cost = (PLAYER.get_car().dur.real * 10) as i32;
                    if PLAYER.get_money() >= cost {
                        PLAYER.car.dur.real += 1;
                        PLAYER.wallet.sub(cost);
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
                if PLAYER.get_car().cgo.real < PLAYER.get_car().cgo.max {
                    let cost = (PLAYER.get_car().cgo.real * 10) as i32;
                    if PLAYER.get_money() >= cost {
                        PLAYER.car.cgo.real += 1;
                        PLAYER.wallet.sub(cost);
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
                if PLAYER.get_still().spd.real < PLAYER.get_still().spd.max {
                    let cost = (PLAYER.get_still().spd.real * 10) as i32;
                    if PLAYER.get_money() >= cost {
                        PLAYER.still.spd.real += 1;
                        PLAYER.wallet.sub(cost);
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
                if PLAYER.get_still().vol.real < PLAYER.get_still().vol.max {
                    let cost = (PLAYER.get_still().vol.real * 10) as i32;
                    if PLAYER.get_money() >= cost {
                        PLAYER.still.vol.real += 1;
                        PLAYER.wallet.sub(cost);
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
                if PLAYER.get_still().qlt.real < PLAYER.get_still().qlt.max {
                    let cost = (PLAYER.get_still().qlt.real * 10) as i32;
                    if PLAYER.get_money() >= cost {
                        PLAYER.still.qlt.real += 1;
                        PLAYER.wallet.sub(cost);
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

fn chase(route: Route, mut distance_traveled: u32, heat: u32) -> (i32, Route) {
    SetForegroundColor(crossterm::style::Color::Red);
    // Initialize Chase & Calculate number of Rolls till Blockade
    let route_distance = route.distance;
    let mut num_rolls_left =
        1 + (route_distance - distance_traveled) / (PLAYER.get_car().spd.real + 3); // was divided by speed + 4

    // Start Loop
    while num_rolls_left > 0 {
        // Header n Stuff
        print_header(2);
        println!(
            "| {} The cops are forming a {} in {} rolls!",
            "STATUS: ".cyan(),
            "BLOCKADE".red(),
            num_rolls_left
        );
        print_separator();
        print_drive_stage(distance_traveled, route_distance, &route.name);

        // Roll Dice & Prompt for player input
        let die1: u8 = get_random_number(6) as u8;
        let die2: u8 = get_random_number(6) as u8;
        let police_roll: u8 = get_random_number(6) as u8;
        print_roll(die1, die2);
        print_roll_prompt(false, die1, die2);
        // Get & Process the player input
        let other_die: u8;
        let die_choice = get_instant_input(&['1', '2']).unwrap().to_digit(10);
        match die_choice.unwrap() {
            1 => {
                distance_traveled += die1 as u32 + PLAYER.get_car().spd.real;
                other_die = die2;
                1
            }
            2 => {
                distance_traveled += die2 as u32 + PLAYER.get_car().spd.real;
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
        print_header(2);
        println!(
            "| {} The cops are forming a {} in {} rolls!",
            "STATUS: ".cyan(),
            "BLOCKADE".red(),
            num_rolls_left
        );
        print_separator();
        print_drive_stage(distance_traveled, route_distance, &route.name);
        print_roll(die1, die2); // Should CROSS OUT die that's been used
                                // Give Info
        print_separator();
        println!("");
        println!(
            "| {}:\t\t\t[{}] {} + {} -> {} Total Attack",
            "COPS' ROLL".cyan().bold(),
            die_from_u8(police_roll),
            police_roll,
            heat,
            ((police_roll + heat as u8).to_string()).red().bold()
        );
        println!(
            "| {}: \t[{}] {} + {} -> {} Total Defense Score",
            "YOUR DEFENSE ROLL".cyan().bold(),
            die_from_u8(other_die),
            other_die,
            PLAYER.get_car().inc.real,
            ((other_die as u32 + PLAYER.get_car().inc.real).to_string())
                .green()
                .bold()
        );
        println!();
        if distance_traveled >= route_distance {
            println!("{}! You made it just in time!", "Hoo Wee".bold().green());
            return ((PLAYER.get_car().current_durability as i32), route);
        }
        // Give Results
        let total_heat: u32 = police_roll as u32 + heat;
        if other_die as u32 + PLAYER.get_car().inc.real < total_heat {
            println!(
                "| Cops' Attack exceeds your Defense:\n| {}",
                "You've been rammed!".red()
            );
            unsafe {
                PLAYER.car.current_durability -= 1;
            }
            if PLAYER.get_car().current_durability == 0 {
                return (0, route);
            }
        } else if distance_traveled < route_distance {
            println!("| {}", "You might just make it yet!".green());
        }
        num_rolls_left -= 1;
        if num_rolls_left == 0 && distance_traveled < route_distance {
            if emergency_roll() < 10 {
                return (-1, route);
            } else {
                println!("{}", "YOU MADE IT!!".bold().green());
                return ((PLAYER.get_car().current_durability as i32), route);
            }
        }
        prompt_to_continue(None);
        clear();
    }
    ((PLAYER.get_car().current_durability as i32), route)
}

fn emergency_roll() -> i32 {
    println!("The cops have formed their blockade - you might be a goner! Roll those dice and pray, Gambler!\nRemember: you need to roll a {} or you're {}.", "10".bold().red(), "BUSTED".bold().red());
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
    println!("Choose from the available routes. If a route's heat is higher than your car's Incognito Score (INC: {}), it'll be risky!\n", PLAYER.get_car().inc.real);
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
    let random_number: i32 = rng.gen_range(0.._d) + 1;
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
    crossterm::terminal::enable_raw_mode();
    if let Ok(Event::Key(key_event)) = read() {
        match key_event.code {
            KeyCode::Char(c) => {
                crossterm::terminal::disable_raw_mode();
                return Some(c);
            }
            KeyCode::Esc => {
                if crossterm::terminal::is_raw_mode_enabled().unwrap() {
                    crossterm::terminal::disable_raw_mode();
                }
                quit();
                None
            }
            _ => None,
        }
    } else {
        None
    }
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

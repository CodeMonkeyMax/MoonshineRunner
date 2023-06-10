use crossterm::event::{self, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use lib::{car::Car, player::Player, route::Route, stat::Stat, still::Still, sutil::*};
use rand::Rng;
use std::io;

pub mod lib;

pub static MAX_STAT: u32 = 24;
pub static CAR_STAT_LENGTH: u8 = 12;
pub static MONEY_MULT: f64 = 10.0;

////////////////////////////////////////////////////////////////////////////////////////
// MAIN ////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    let mut player: Player = start();
    let mut quit = false;

    // initialization done. Do your tesing here.

    print_solo("Are you ready, gambler?".to_string());
    prompt_to_continue(None);

    while !quit {
        print_solo("Welcome to Moonshine Runner!".to_string());
        let mut end_round = false;
        while !end_round {
            // BREW
            prompt_to_continue(Some("brew stage".to_string()));
            brew(&mut player);

            // DRIVE
            prompt_to_continue(Some("drive stage".to_string()));
            let cargo_status = drive(&mut player);
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
    print_header(player, 1);
    //take into account player's still size and quality
    let odds = match player.still.qlt.real {
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
        v if v < odds.0 => 1,
        v if v < odds.0 + odds.1 => 2,
        _ => 3,
    };
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
    let route = choose_route();
    let route_distance = route.distance;
    let heat = route.heat;

    let mut distance_traveled: u32 = 0;
    let player_still_alive = true;
    let mut player_spotted = false;

    print_header(player, 2);
    println!("| Start your engine, gambler!");
    prompt_to_continue(None);
    while distance_traveled < route_distance && player_still_alive {
        // Initialize the round
        if distance_traveled > route_distance {
            distance_traveled = route_distance;
        }
        // Print Header
        print_header(player, 2);
        // Check if the player has been spotted
        if player_spotted {
            if distance_traveled == route_distance {
                println!("Ha! You squeaked right by 'em!");
                return player.car.dur.real as i32 + 1;
            }
            let chase_result = chase(&mut player, route, distance_traveled, heat);
            match chase_result {
                -1 => println!("Your whiskey and your ride are history, Gambler!"),
                0 => println!("You barely made it outta there! Good thing you talked 'em into returning your keys!'"),
                _ => println!("Holy Hockey Sticks! You made it by the seat of your pants!"),
            }
            return chase_result;
        }

        // Part 1 - Start of Round Stage
        print_drive_stage(player, distance_traveled, route_distance, &route.name);
        prompt_to_continue(Some("roll".to_string()));
        let die1: u8 = get_random_number(6) as u8;
        let die2: u8 = get_random_number(6) as u8;
        let police_roll: u8 = get_random_number(6) as u8;

        // Part 2 - Roll Dice

        print_header(player, 2);
        if distance_traveled >= 1 {
            println!(
                "| {} Good work, Gambler. They haven't spotted you yet!",
                "STATUS: ".cyan()
            );
            print_separator();
        }
        print_drive_stage(player, distance_traveled, route_distance, &route.name);

        print_roll(die1, die2);
        // Have Player Choose Die
        print_roll_prompt(player, true, die1, die2);
        // Process Consequences of Die Choice
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

        // Part 3 - Consequences, John.
        print_header(player, 2);
        if distance_traveled >= 1 {
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
            player.car.inc.real,
            other_die as u32 + player.car.inc.real
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
        if other_die as u32 + player.car.inc.real < total_heat
            && distance_traveled != route_distance
        {
            print_solo_bad("You've been made! Floor it!".to_string());
            prompt_to_continue(Some("roll".to_string()));
            player_spotted = true;
        }
    }
    print_solo("You made it!".to_string());
    prompt_to_continue(None);
    return player.car.dur.real as i32 + 1;
}

fn barter(mut player: &mut Player, cargo_status: i32) {
    let mut mult: f64 = 1.0;
    print_header(player, 3);
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
        "You rolled a {}. Given this and the results of your drive, you get: ${}",
        die, money_increment as i32
    );
    player.money += money_increment as i32;
}

fn buy(player: &mut Player) {
    print_header(player, 4);
    println!("Visit the Police Auction? (y/n)");
    let mut user_entry: Option<char> = get_valid_input(&['y', 'n']);
    if user_entry == Some('y') {
        auction_house(player);
    }
    print_header(player, 4);
    println!("Do you want to buy any upgrades? (y/n)");
    let mut user_entry: Option<char> = get_valid_input(&['y', 'n']);
    while user_entry != Some('n') {
        println!("Do you want to buy upgrades for your CAR or for your STILL? (c/s)");
        if shop_category(player, get_valid_input(&['c', 's']).unwrap()) {
            println!("Congratulations on your snazzy new upgrade.");
        }
        println!("Shop Again? (y/n)");
        user_entry = get_valid_input(&['y', 'n']);
    }
    if user_entry == Some('y') {}
}

///////////////////////////////////////////////////////////////////////////////////////////
// TIER 3  ////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////

fn auction_house(mut player: &mut Player) {
    print_header(player, -1);
    let car1: Car = Car::generate(1);
    let car2: Car = Car::generate(2);
    let car3: Car = Car::generate(3);
    println!(
        "| Available Cars: \n{}\n{}\n{}",
        car1.to_string(),
        car2.to_string(),
        car3.to_string()
    );
    println!("| Imagine how cool this will be once you can buy cars!");
    prompt_to_continue(None);
}

fn shop_category(mut player: &mut Player, category_code: char) -> bool {
    let print_msg: String = format!("Your Money: {}", player.money);
    println!("{}", print_msg.green());
    match category_code {
        'c' => {
            println!("OK, let's look at some car upgrades.");
            println!(
                "Do you want to upgrade:\nSPEED (${}), DURABILITY (${}) or CARGO (${})? (s/d/c)",
                player.car.spd.real * 10,
                player.car.dur.real * 10,
                player.car.cgo.real * 10
            );
            return try_buy(
                player,
                category_code,
                get_valid_input(&['s', 'd', 'c']).unwrap(),
            );
        }
        's' => {
            println!("OK, let's look at some still upgrades.");
            println!(
                "Do you want to upgrade:\nVOLUME (${}), SPEED (${}) or QUALITY (${})? (v/s/q)",
                player.still.vol.real * 10,
                player.still.spd.real * 10,
                player.still.qlt.real * 10
            );
            return try_buy(
                player,
                category_code,
                get_valid_input(&['v', 's', 'q']).unwrap(),
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
                    println!("You're already maxed out, speed racer!");
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
                    println!("Woah there Pappy, you're already maxed out!");
                }
                return false;
            }
            'v' => {
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
                    println!("Woah there Pappy, you're already maxed out!");
                }
                return false;
            }
            'q' => {
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

fn chase(mut player: &mut Player, route: Route, mut distance_traveled: u32, heat: u32) -> i32 {
    // Initialize Chase & Calculate number of Rolls till Blockade
    let route_distance = route.distance;
    let mut num_rolls_left = 1 + (route_distance - distance_traveled) / (player.car.spd.real + 4);
    // Get Player Car Durability
    let mut current_durability = player.car.dur.real;

    // Start Loop
    while num_rolls_left > 0 {
        // Header n Stuff
        print_header(player, 2);
        println!(
            "| {} The cops are forming a {} in {} rolls!",
            "STATUS: ".cyan(),
            "BLOCKADE".red(),
            num_rolls_left
        );
        print_separator();
        print_drive_stage(player, distance_traveled, route_distance, &route.name);

        // Roll Dice & Prompt for player input
        let die1: u8 = get_random_number(6) as u8;
        let die2: u8 = get_random_number(6) as u8;
        let police_roll: u8 = get_random_number(6) as u8;
        print_roll(die1, die2);
        print_roll_prompt(player, true, die1, die2);
        // Get & Process the player input
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

        // Consequences...
        print_header(player, 2);
        print_drive_stage(player, distance_traveled, route_distance, &route.name);
        // Give Info
        println!(
            "| Cops' Roll:\t\t\t[{}] {} + {} -> {} Total Attack",
            die_from_u8(police_roll),
            police_roll,
            heat,
            ((police_roll + heat as u8).to_string()).red().bold()
        );
        println!(
            "| Your Defense Roll: \t[{}] {} + {} -> {} Total Defense Score",
            die_from_u8(other_die),
            other_die,
            player.car.inc.real,
            ((other_die as u32 + player.car.inc.real).to_string())
                .green()
                .bold()
        );
        // Give Results
        let total_heat: u32 = police_roll as u32 + heat;
        if other_die as u32 + player.car.inc.real < total_heat {
            println!(
                "| Cops' Attack exceeds your Defense:\n| {}",
                "You've been rammed!".red()
            );
            current_durability -= 1;
            if current_durability == 0 {
                return 0;
            }
        } else if distance_traveled < route_distance {
            println!("| {}", "You might just make it yet!".green());
        }
        num_rolls_left -= 1;
        if num_rolls_left == 0 && distance_traveled < route_distance {
            if emergency_roll() < 10 {
                return -1;
            } else {
                println!("{}", "YOU MADE IT!!".bold().green());
                return current_durability as i32;
            }
        }
        prompt_to_continue(None);
        clear();
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
    println!("\tAvailable Routes:\n");
    let mut routes_fields: Vec<Vec<String>> = Vec::new();

    let mut i = 0;
    for route_index in &drawn_routes {
        let mut result: Vec<String> = Vec::new();
        i = i + 1;
        result.push(format!("Route #{}:     \t", i.to_string().green()));
        for field in routes.get(*route_index).unwrap().clone().get_all_fields() {
            result.push(field.to_string());
        }
        routes_fields.push(result);
    }

    for line_num in 0..9 {
        for column in 0..3 {
            print!(
                "| {:<18}\t",
                routes_fields.get(column).unwrap().get(line_num).unwrap()
            );
        }
        print!(" |");
        println!();
    }
    // Give route options
    println!("\n\t{}", "Pick a route (1/2/3):".cyan().bold());
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

use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    cursor, execute,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::io::{stdout, Write};

pub fn progress_bar(real: u32, max: u32) -> String {
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
pub fn progress_bar_upgradeable(mut real: u32, mut max: u32, scale: u32) -> String {
    // Resize bar if values are greater than scale witdh
    // WAS real + max, but that's... Wrong, I think?
    let sum = max;
    let mut factor = 0;
    if sum != 0 {
        // can't divide a negative unsigned int!
        factor = (sum - 1) / scale; // if 0, sum is less than scale. '-1' stops a max value from triggering a resize. If 1, sum is greater, so size should double (by halving values).
    }
    //println!("Real: {} Max: {} Factor: {}", real, max, factor);
    real = real / (factor + 1);
    max = max / (factor + 1);
    //println!("New Real: {} New Max: {}", real, max);

    let mut result: String = String::from("[");
    for _i in 0..real {
        result.push('=');
    }
    for _i in real..max {
        result.push('-');
    }
    for _i in max..scale {
        result.push(' ');
    }
    result.push(']');
    result
}
pub fn progress_bar_head(mut real: u32, max: u32) -> String {
    let mut result: String = String::from("[");
    if real < 1 {
        real = 1
    };
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
pub fn print_progress_bar_head(mut real: u32, max: u32) {
    print!("[");
    if real < 1 {
        real = 1
    };
    for _i in 0..real - 1 {
        print!("{}", "=".bold().yellow());
    }
    print!("{}", "#".bold().green());
    for _i in real..max {
        print!("{}", "-".bold());
    }
    print!("]");
}
pub fn die_from_u8(num: u8) -> char {
    match num {
        1 => '⚀',
        2 => '⚁',
        3 => '⚂',
        4 => '⚃',
        5 => '⚄',
        6 => '⚅',
        _ => panic!("BAD NUMBER!"),
    }
}
pub fn print_header(player: &mut crate::Player, round_stage: i32) {
    clear();
    print_separator();
    println!("| [{}] to {} |   |  {} | {:<6}",
        "ESC".bold(),
        "quit".red(),
        "MOONSHINE RUNNER ".bold().blue(),
        "♦️ ♥️ ♣️ ♠️ ♦️ ♥️ ♣️ ♠️ ♦️ ♥️ ♣️");
    print_separator();
    print_money_progress(player.money as u32, 6000);
    print_separator();
    let mut message_components: Vec<String> = vec![];
    match round_stage {
        1 => {
            message_components.push("| STAGE 1 of 4:".to_string()); // should use '║'
            message_components.push("
|\t ___ ___ _____      __  ___ _____ _   ___ ___ 
|\t| _ ) _ \\ __\\ \\    / / / __|_   _/_\\ / __| __|
|\t| _ \\   / _| \\ \\/\\/ /  \\__ \\ | |/ _ \\ (_ | _| 
|\t|___/_|_\\___| \\_/\\_/   |___/ |_/_/ \\_\\___|___|\n|\n".to_string());
            message_components
                .push("| Alright gambler, let's brew some backyard shine.\n".to_string());
        }
        2 => {
            message_components.push("| STAGE 2 of 4:".to_string());
            message_components.push("
|\t ___  ___ _____   _____   ___ _____ _   ___ ___ 
|\t|   \\| _ \\_ _\\ \\ / / __| / __|_   _/_\\ / __| __|
|\t| |) |   /| | \\ V /| _|  \\__ \\ | |/ _ \\ (_ | _|
|\t|___/|_|_\\___| \\_/ |___| |___/ |_/_/ \\_\\___|___|\n|\n".to_string());
            message_components.push("| Buckle up, Gambler. It's time to drive!\n".to_string());
        }
        3 => {
            message_components.push("| STAGE 3 of 4:".to_string());
            message_components.push("
|\t ___   _   ___ _____ ___ ___   ___ _____ _   ___ ___ 
|\t| _ ) /_\\ | _ \\_   _| __| _ \\ / __|_   _/_\\ / __| __|
|\t| _ \\/ _ \\|   / | | | _||   / \\__ \\ | |/ _ \\ (_ | _| 
|\t|___/_/ \\_\\_|_\\ |_| |___|_|_\\ |___/ |_/_/ \\_\\___|___|\n|\n".to_string());
        }
        4 => {
            message_components.push("| STAGE 4 of 4:".to_string());
            message_components.push("
|\t ___ _   ___   __  ___ _____ _   ___ ___ 
|\t| _ ) | | \\ \\ / / / __|_   _/_\\ / __| __|
|\t| _ \\ |_| |\\ V /  \\__ \\ | |/ _ \\ (_ | _| 
|\t|___/\\___/  |_|   |___/ |_/_/ \\_\\___|___|\n|\n".to_string());
            message_components.push("+$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$".to_string());
            message_components.push("$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$$+\n".to_string());
            message_components.push(format!("| Your Car:\n| {}\n", player.car.to_string(),));
            message_components.push("+===+===+===+===+===+===+===+===+====".to_string());
            message_components.push("====+===+===+===+===+===+===+===+===+\n".to_string());
            message_components.push(format!(
                "| {}:\n{}\n",
                "Your Still".cyan().bold(),
                player.still.to_string()
            ));
        }
        _ => ()
    }

    message_components.push(format!("| Cash: ${}\n", player.money));

    for component in message_components {
        print!("{}", component);
    }

    print_separator();
}

pub fn print_roll(die1: u8, die2: u8) {
    // Progress Bar
    println!(
        "| {}:| {}:| <--- YOUR\n| {} [{}]\t| {} [{}]\t| <--- DICE",
        "Die 1".yellow().bold(),
        "Die 2".yellow().bold(),
        die1.to_string().yellow(),
        die_from_u8(die1),
        die2.to_string().yellow(),
        die_from_u8(die2)
    );
    if die1 + die2 == 2 {
        println!("| {}", "Snake eyes!".red().bold())
    }
}

pub fn print_separator() {
    println!("+===+===+===+===+===+===+===+===+========+===+===+===+===+===+===+===+===+");
}

pub fn print_roll_prompt(player: &mut crate::Player, player_is_hidden: bool, die1: u8, die2: u8) {
    if player_is_hidden {
        println!(
            "| You can apply {} to your {} stat & get there quicker.\n| The other will be applied to your {}, keeping you under\n| the radar for longer. Choose wisely...",
            "one die".cyan(),
            "speed".green(),
            //"one".cyan(),
            "stealth".yellow()
        );
        println!(
            "\nWhich roll do you want to apply to your {}? {}\n",
            "SPEED".cyan().bold(),
            "(1/2)".bold().cyan()
        );
        let sum1 = (die1 as u32 + player.car.spd.real).to_string();
        println!(
            "| {}: [{}] {} + {} -> {} Total Speed",
            "Die 1".cyan(),
            die1,
            die_from_u8(die1),
            player.car.spd.real,
            sum1.bold().yellow()
        );
        let sum2 = (die2 as u32 + player.car.spd.real).to_string();
        println!(
            "| {}: [{}] {} + {} -> {} Total Speed",
            "Die 2".cyan(),
            die2,
            die_from_u8(die2),
            player.car.spd.real,
            sum2.bold().yellow()
        );
    } else {
        println!(
            "| You can use {} to go {}, one to {} the cops' pit maneuvers.",
            "one roll".cyan(),
            "faster".green(),
            "evade".yellow()
        );
        println!(
            "\n\tWhich roll do you want to apply to your {}? {}\n",
            "SPEED".cyan().bold(),
            "(1/2)".bold().cyan()
        );
        let sum1 = (die1 as u32 + player.car.spd.real).to_string();
        println!(
            "| {}: [{}] {} + {} -> {} Total Speed",
            "Die 1".cyan(),
            die1,
            die_from_u8(die1),
            player.car.spd.real,
            sum1.bold().yellow()
        );
        let sum2 = (die2 as u32 + player.car.spd.real).to_string();
        println!(
            "| {}: [{}] {} + {} -> {} Total Speed",
            "Die 2".cyan(),
            die2,
            die_from_u8(die2),
            player.car.spd.real,
            sum2.bold().yellow()
        );
    }
}

pub fn print_money_progress(real: u32, max: u32) {
    let sum = max;
    let mut factor = 0;
    if sum != 0 {
        // can't divide a negative unsigned int!
        factor = (sum - 1) / 42; // if 0, sum is less than scale. '-1' stops a max value from triggering a resize. If 1, sum is greater, so size should double (by halving values).
    }
    //println!("Real: {} Max: {} Factor: {}", real, max, factor);
    let _real = real / (factor + 1);
    let _max = max / (factor + 1);
    print!("| {}: [","Your Progress".green().bold());
    for _i in 0.._real {
        print!("{}","$".green());
    }
    for _i in _real.._max {
        print!("{}","$".dim());
    }
    print!("]");
    print!(" ${} / ${}", real.to_string().green(), max.to_string().bold());
    println!();
}

pub fn clear() {
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
}

pub fn print_solo(message: String) {
    clear();
    println!("\t{}\n\n", message.cyan().bold());
}

pub fn print_solo_bad(message: String) {
    clear();
    println!("\t{}\n\n", message.red().bold());
}

// Print Stages
pub fn print_brew_stage(player: &mut crate::Player) {
    println!(
        "| {}:\n{}\n|",
        "Your Still".cyan().bold(),
        player.still.to_string()
    );
    println!("|   /-----------\\");
    println!("|  (|-----------|)");
    println!("|  (|-----------|)");
    println!("|  (|----/ \\----|)");
    println!("|   \\-----------/");
    println!("|");
    println!("| Brewing...");
}
pub fn print_drive_info(
    player: &crate::Player,
    distance_traveled: u32,
    route_distance: u32,
    route_name: &String,
) {
    // Print Car
    println!("| {}: {}", "Your Car".bold().cyan(), player.car.to_string());
    print_separator();
    // Print Route Progress
    println!(
        "| {}:\n| {}",
        "Your Progress".bold().yellow(),
        (route_name.clone()).red()
    );
    print!("| ");
    print_progress_bar_head(distance_traveled, route_distance);
    print!("{}/{}\n", distance_traveled, route_distance);
    print_separator();
}
pub fn print_barter_stage(player: &mut crate::Player) {}
pub fn print_buy_stage(player: &mut crate::Player) {}

pub fn print_barn() {
    println!(
        "                             +&-
                           _.-^-._    .--.
                        .-'   _   '-. |__|
                       /     |_|     \\|  |
                      /               \\  |
                     /|     _____     |\\ |
                      |    |==|==|    |  |
  |---|---|---|---|---|    |--|--|    |  |
  |---|---|---|---|---|    |==|==|    |  |
 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^"
    );
}

pub fn print_bottles() {
    println!(
        "\r
\t    ___    ____
\t   |___|  (____)
\t    | |   _)  (_
\t  _/   \\_/~~~~~~\\
\t /~~~~~~/        \\
\t(______(   SHINE  )
\t | XXX  \\        /
\t |______ )______(
\t(_______/________\\"
    );
}

pub fn print_white_lightning() {
    println!(
        "\r
        ,/
     ,'/
   ,' /_,
 .'_  ,'
   /,'
  /'
 '
 "
    )
}

use crate::PLAYER;
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
pub fn print_header(round_stage: i32) {
    clear();
    let mut message_components: Vec<String> = vec![];
    message_components.push("\t\t\t\t - - ♠️ ♣️ ♥️ ♦️ MOONSHINE RUNNER ♦️ ♥️ ♣️ ♠️ - -\n".to_string());
    message_components.push("+===+===+===+===+===+===+===+===+====".to_string());
    message_components.push("====+===+===+===+===+===+===+===+===+\n".to_string());
    match round_stage {
        1 => {
            message_components.push("| STAGE 1 of 4: BREW\n".to_string()); // should use '║'
            message_components
                .push("| Alright gambler, let's brew some backyard shine.\n".to_string());
        }
        2 => {
            message_components.push("| STAGE 2 of 4: DRIVE\n".to_string());
            message_components.push("| Buckle up, Gambler. It's time to drive!\n".to_string());
        }
        3 => {
            message_components.push("| STAGE 3 of 4: BARTER\n".to_string());
        }
        4 => {
            message_components.push("| STAGE 4 of 4: BUY\n".to_string());
            message_components.push("| Time to buy something!\n".to_string());
            message_components.push(format!(
                "| Your Car:\n{}\n",
                crate::PLAYER.get_car().to_string(),
            ));
            message_components.push("+===+===+===+===+===+===+===+===+====".to_string());
            message_components.push("====+===+===+===+===+===+===+===+===+\n".to_string());
            message_components.push(format!(
                "| {}: {}\n",
                "Your Still".cyan().bold(),
                crate::PLAYER.get_still().to_string()
            ));
        }
        _ => {
            message_components.push("|\n".to_string());
        }
    }

    message_components.push(format!("| Cash: ${}\n", crate::PLAYER.get_money()));
    message_components.push("+===+===+===+===+===+===+===+===+====".to_string());
    message_components.push("====+===+===+===+===+===+===+===+===+\n".to_string());

    for component in message_components {
        print!("{}", component);
    }
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

pub fn print_roll_prompt(player_is_hidden: bool, die1: u8, die2: u8) {
    if player_is_hidden {
        println!(
            "| You can apply {} to your {} stat & get there quicker.\n| The other will be applied to your {}, keeping you under\n| the radar for longer. Choose wisely...",
            "one die".cyan(),
            "speed".green(),
            //"one".cyan(),
            "stealth".yellow()
        );
        println!(
            "\n\tWhich roll do you want to apply to your {}? {}\n",
            "SPEED".cyan().bold(),
            "(1/2)".bold().cyan()
        );
        let sum1 = (die1 as u32 + PLAYER.get_car().spd.real).to_string();
        println!(
            "| {}: [{}] {} + {} -> {} Total Speed",
            "Die 1".cyan(),
            die1,
            die_from_u8(die1),
            PLAYER.get_car().spd.real,
            sum1.bold().yellow()
        );
        let sum2 = (die2 as u32 + PLAYER.get_car().spd.real).to_string();
        println!(
            "| {}: [{}] {} + {} -> {} Total Speed",
            "Die 2".cyan(),
            die2,
            die_from_u8(die2),
            PLAYER.get_car().spd.real,
            sum2.bold().yellow()
        );
    } else {
        println!(
            "| You can use {} to go {}, and one to {} the cops' pit maneuvers.",
            "one roll".cyan(),
            "faster".green(),
            "evade".yellow()
        );
        println!(
            "\n\tWhich roll do you want to apply to your {}? {}\n",
            "SPEED".cyan().bold(),
            "(1/2)".bold().cyan()
        );
        let sum1 = (die1 as u32 + PLAYER.get_car().spd.real).to_string();
        println!(
            "| {}: [{}] {} + {} -> {} Total Speed",
            "Die 1".cyan(),
            die1,
            die_from_u8(die1),
            PLAYER.get_car().spd.real,
            sum1.bold().yellow()
        );
        let sum2 = (die2 as u32 + PLAYER.get_car().spd.real).to_string();
        println!(
            "| {}: [{}] {} + {} -> {} Total Speed",
            "Die 2".cyan(),
            die2,
            die_from_u8(die2),
            PLAYER.get_car().spd.real,
            sum2.bold().yellow()
        );
    }
}

pub fn clear() {
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
}

pub fn print_solo(message: String) {
    clear();
    println!("\t\t{}\n\n", message.cyan().bold());
}

pub fn print_solo_bad(message: String) {
    clear();
    println!("\t\t{}\n\n", message.red().bold());
}

// Print Stages
pub fn print_brew_stage() {
    println!(
        "| {}:\n{}\n|",
        "Your Still".cyan().bold(),
        PLAYER.get_still().to_string()
    );
    println!("|   /-----------\\");
    println!("|  (|-----------|)");
    println!("|  (|-----------|)");
    println!("|  (|----/ \\----|)");
    println!("|   \\-----------/");
    println!("|");
    println!("| Brewing...");
}
pub fn print_drive_stage(distance_traveled: u32, route_distance: u32, route_name: &String) {
    // Print Car
    println!(
        "| {}: {}",
        "Your Car".bold().cyan(),
        PLAYER.get_car().to_string()
    );
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
pub fn print_barter_stage() {}
pub fn print_buy_stage() {}

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

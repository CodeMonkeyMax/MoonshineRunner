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
    let sum = real + max;
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

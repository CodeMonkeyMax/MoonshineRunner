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
pub fn progress_bar_upgradeable(real: u32, max: u32, scale: u32) -> String {
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
pub fn progress_bar_head(real: u32, max: u32) -> String {
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

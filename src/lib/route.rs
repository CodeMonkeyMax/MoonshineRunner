use crossterm::style::Stylize;

pub struct Route {
    pub name: String,
    pub distance: u32,
    pub heat: u32,
    pub prefereces: Vec<u32>,
    pub prices: Vec<u32>,
}
impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:\nDistance: {}\tHeat: {}\nPreferences:\nRotgut Wiskee: {}\nOK Hooch: {}\nWhite Lightning: {}",
					&self.name.to_string().blue(),
					self.distance,
					self.heat,
					self.prefereces[0],
					self.prefereces[1],
					self.prefereces[2],
        )
    }
}

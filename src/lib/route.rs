use crossterm::style::Stylize;

#[derive(Debug, Clone)]
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
            "{}:\nDistance: {}\tHeat: {}\n-----------------------\nPreferences:\nRotgut Wiskee: {}\nOK Hooch: {}\nWhite Lightning: {}",
					&self.name.to_string().blue(),
					self.distance,
					self.heat,
					self.prices[0],
					self.prices[1],
					self.prices[2],
        )
    }
}
impl Route {
    pub fn get_all_fields(self) -> Vec<String> {
        let mut fields: Vec<String> = Vec::new();
        fields.push(String::from(self.name));
        fields.push("   ---------------".to_string());
        fields.push(format!(
            "{:<18} {:>2}",
            "Distance:",
            self.distance.to_string()
        ));
        fields.push(format!("{:<18} {:>2}", "Heat:", self.heat.to_string()));
        fields.push("Preferences: ".to_string());
        fields.push(format!(
            "{:<18}{:>3}",
            "Rotgut Wiskee:",
            format!("${}", self.prices[0].to_string())
        ));
        fields.push(format!(
            "{:<18}{:>3}",
            "OK Hooch:",
            format!("${}", self.prices[1].to_string())
        ));
        fields.push(format!(
            "{:<18}{:>3}",
            "White Lightning:",
            format!("${}", self.prices[2].to_string())
        ));
        return fields;
    }
}

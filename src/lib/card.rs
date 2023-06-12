use core::fmt::Display;
use crossterm::style::{Stylize, StyledContent};

pub struct Card {
	pub rows: i32,
	pub cols: i32,
	pub art: Vec<StyledContent<String>>,
}

pub const SMALL_TEST_CARD: Card = Card::new(vec!["{}"," ".cyan()]);
pub const MEDIUM_TEST_CARD: Card = Card::new(vec!["  ", "  "]);
pub const LARGE_TEST_CARD: Card = Card::new(vec!["   ", "   ", "   "]);
pub const JAGGED_TEST_CARD: Card = Card::new(vec!["   ", "   ", "  "]);
pub const BIG_RIG: Card = Card::new(vec![
	"         /-/--|			",
	"	 /^\\---/_/| +|			",
	" *|#|*/-\\__|__|______===_",
	" ====((O)-----)_)_)((O)(O)",
]);

impl Card {
	pub fn new(art: Vec<StyledContent<String>>) -> Self {
		let rows = art.len();
		let mut cols = 0;
		let art = art;
		while let i = 0 < rows {
			if art[i].len() > cols {
				cols = art[0].content.len();
			}
		}
		Card {
			rows,
			cols,
			art,
		}
	}
	pub fn get(&self) -> StyledContent<String> {
		let mut constructed_card = StyledContent::new(crossterm::style::ContentStyle::new(), "");
		// Create Top Border
		constructed_card.content.push_str("╔");
		for i in 1..self.cols {
			constructed_card.content.push_str("═");
		}
		constructed_card.content.push_str("╗");
		// Create Body
		for row in 1..self.rows {
			constructed_card.content.push_str("║");
			constructed_card.content.push(self.art[row]);
			constructed_card.content.push_str("║");
		}
		// Create Bottom Border
		constructed_card.content.push_str("╚");
		for i in 1..self.cols {
			constructed_card.content.push_str("═");
		}
		constructed_card.content.push_str("╝");
	}
}
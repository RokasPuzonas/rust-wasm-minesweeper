use std::{collections::HashSet, fmt::{Display, Write}};

use rand::Rng;

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Minesweeper {
	width: usize,
	height: usize,
	open_fields: HashSet<Position>,
	mines: HashSet<Position>,
	flags: HashSet<Position>,
	game_over: bool
}

impl Display for Minesweeper {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for y in 0..self.height {
			for x in 0..self.height {
				let pos = (x, y);
				if self.game_over && self.mines.contains(&pos) {
					f.write_str("B ")?;
				} else if self.flags.contains(&pos) {
					f.write_str("F ")?;
				} else if !self.open_fields.contains(&pos) {
					f.write_str("# ")?;
				} else if self.mines.contains(&pos) {
					f.write_str("B ")?;
				} else {
					let mines = self.count_mines(pos);
					if mines == 0 {
						write!(f, "☐ ")?;
					} else {
						write!(f, "{} ", mines)?;
					}
				}
			}
			f.write_char('\n')?;
		}

		Ok(())
	}
}

pub enum OpenResult {
	Mine,
	NoMine(u8)
}

impl Minesweeper {
	pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
		Minesweeper {
			width,
			height,
			open_fields: HashSet::new(),
			mines: generate_mines(width, height, mine_count),
			flags: HashSet::new(),
			game_over: false
		}
	}

	pub fn iter_neighbours(&self, (x, y): Position) -> impl Iterator<Item = Position> {
		let width = self.width;
		let height = self.height;

		(x.max(1) - 1 ..= (x + 1).min(width - 1)).flat_map(
			move |i| (y.max(1) - 1 ..= (y + 1).min(height - 1)).map(move |j| (i, j))
		).filter(move |&pos| pos != (x, y))
	}

	pub fn count_mines(&self, position: Position) -> u8 {
		self.iter_neighbours(position)
			.filter(|pos| self.mines.contains(pos))
			.count() as u8
	}

	pub fn open(&mut self, position: Position) -> Option<OpenResult> {
		if self.game_over || self.open_fields.contains(&position) || self.flags.contains(&position) {
			return None;
		}

		self.open_fields.insert(position);
		let is_mine = self.mines.contains(&position);
		if is_mine {
			self.game_over = true;
			Some(OpenResult::Mine)
		} else {
			let mines = self.count_mines(position);
			if mines == 0 {
				for neighbour in self.iter_neighbours(position) {
					self.open(neighbour);
				}
			}
			Some(OpenResult::NoMine(mines))
		}
	}

	pub fn toggle_flag(&mut self, position: Position) {
		if self.game_over || self.open_fields.contains(&position) {
			return;
		}

		if self.flags.contains(&position) {
			self.flags.remove(&position);
		} else {
			self.flags.insert(position);
		}
	}
}

fn generate_mines(field_width: usize, field_height: usize, mine_count: usize) -> HashSet<Position> {
	let mut mines = HashSet::new();

	let mut rng = rand::thread_rng();
	while mines.len() < mine_count {
		let x = rng.gen_range(0..field_width);
		let y = rng.gen_range(0..field_height);
		mines.insert((x, y));
	}

	mines
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut ms = Minesweeper::new(5, 5, 5);

		ms.open((2, 2));
		ms.toggle_flag((3, 3));

		println!("{}", ms);
	}
}

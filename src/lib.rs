#![allow(unused_unsafe)]
mod minesweeper;

use std::cell::RefCell;

use minesweeper::*;
use wasm_bindgen::prelude::*;

thread_local! {
	static MINESWEEPER: RefCell<Minesweeper> = RefCell::new(Minesweeper::new(10, 10, 10));
}

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
	MINESWEEPER.with(move |ms| ms.borrow().to_string())
}

#[wasm_bindgen(js_name = openField)]
pub fn open_field(x: usize, y: usize) {
	MINESWEEPER.with(move |ms| ms.borrow_mut().open((x, y)));
}

mod universe;
mod cell;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use universe::Universe;


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    use web_sys::console;


    let universe = Universe::new(500, 500);
    universe.fill_background();
    universe.draw_cells();





    Ok(())
}

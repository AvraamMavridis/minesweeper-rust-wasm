mod universe;
mod cell;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use universe::Universe;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    use web_sys::console;


    let mut universe = Universe::new(500, 500);
    universe.init_cells();
    universe.init_mouse_events();

    Ok(())
}

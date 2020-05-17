mod universe;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use universe::Universe;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {

    let universe = Universe::new(500, 500);


    Ok(())
}

use wasm_bindgen::prelude::*;

#[derive(Clone)]
pub enum CellState {
    Number,
    Bomb,
    Empty,
    Marked,
}


#[derive(Clone)]
pub struct Cell {
    position: (u32, u32),
    size: (u32, u32),
    state: CellState,
    hidden: bool,
}

impl Cell {
    pub fn new(position: (u32, u32), size: (u32, u32), state: CellState) -> Cell {
        Cell {
            position,
            size,
            state,
            hidden: true,
        }
    }

    pub fn position(&self) -> &(u32, u32) {
        &self.position
    }

    pub fn size(&self) -> &(u32, u32) {
        &self.size
    }

    pub fn hidden(&self) -> bool {
        self.hidden
    }

    pub fn set_hidden(&mut self, new_hidden: bool) {
        self.hidden = new_hidden
    }

    pub fn set_state(&mut self, new_state: CellState) {
        self.state = new_state;
    }

    pub fn draw_cell(&self, context:  &web_sys::CanvasRenderingContext2d) -> Result<(), JsValue>{
        context.begin_path();
        context.rect(0.0, 0.0, 10.0, 10.0);
        context.set_fill_style(&JsValue::from("green"));
        context.fill();
        Ok(())
    }
}

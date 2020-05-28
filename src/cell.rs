use wasm_bindgen::prelude::*;

#[derive(Clone, PartialEq)]
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
    neighoured_mines: u32,
}

impl Cell {
    pub fn new(position: (u32, u32), size: (u32, u32), state: CellState) -> Cell {
        Cell {
            position,
            size,
            state,
            hidden: true,
            neighoured_mines: 0,
        }
    }

    pub fn position(&self) -> &(u32, u32) {
        &self.position
    }

    pub fn size(&self) -> &(u32, u32) {
        &self.size
    }

    pub fn get_state(&self) -> &CellState {
        &self.state
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

    pub fn set_neighboured_mines(&mut self, n: u32) -> () {
        self.neighoured_mines = n;
    }

    pub fn get_neighboured_mines(&self) -> u32 {
        self.neighoured_mines
    }

    pub fn is_bomb(&self) -> bool {
        match self.state {
            CellState::Bomb => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.state {
            CellState::Empty => true,
            _ => false,
        }
    }

    pub fn draw_cell(&self, context: &web_sys::CanvasRenderingContext2d) -> () {
        context.begin_path();
        context.rect(self.position.0 as f64, self.position.1 as f64, self.size.0 as f64, self.size.1 as f64);

        let color = match self.state {
            CellState::Bomb => JsValue::from("black"),
            _ => JsValue::from("red")
        };

        context.set_fill_style(&color);
        context.fill();
        context.set_stroke_style(&JsValue::from("black"));
        context.stroke();

        context.set_font("15px Georgia");
        context.set_fill_style(&JsValue::from("black"));
        context.fill_text(&self.neighoured_mines.to_string(), (self.position.0 + 15) as f64, (self.position.1 + 25) as f64)
        ;
    }
}

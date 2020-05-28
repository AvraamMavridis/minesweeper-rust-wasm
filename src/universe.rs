use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use math::round;

use crate::cell::{Cell, CellState};
use wasm_bindgen::__rt::core::fmt::Alignment::Center;

type CellsBoard = Vec<Vec<Cell>>;

const CELL_SIZE: u32 = 50;
const NUMBER_OF_MINES: u32 = 10;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Universe {
    width: u32,
    height: u32,
    context: web_sys::CanvasRenderingContext2d,
    canvas: web_sys::HtmlCanvasElement,
    cells: CellsBoard,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let width = width;
        let height = height;
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        canvas.set_width(width);
        canvas.set_height(height);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let mut cells: CellsBoard = vec![];

        Universe {
            width,
            height,
            context,
            canvas,
            cells,
        }
    }

    /// Returns the width of the universe
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the universe
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn context(&self) -> web_sys::CanvasRenderingContext2d {
        self.context.clone()
    }

    pub fn init_cells(&mut self) -> (()) {
        let mut rng = thread_rng();
        let rows_on_universe = self.height / CELL_SIZE;
        let cells_on_row = self.width / CELL_SIZE;

        for i in 0..cells_on_row {
            let mut row: Vec<Cell> = vec![];
            for j in 0..cells_on_row {
                let cell = Cell::new((i * CELL_SIZE, j * CELL_SIZE), (CELL_SIZE, CELL_SIZE), CellState::Empty);
                row.push(cell);
            }
            self.cells.push(row);
        }

        let mut k = 0;

        // define bombs in the universe
        while k < NUMBER_OF_MINES {
            k = k + 1;
            let mut y = rng.gen_range(0.0, rows_on_universe as f32) as usize;
            let mut x = rng.gen_range(0.0, cells_on_row as f32) as usize;
            self.cells[x][y].set_state(CellState::Bomb);
        }

        for row in 0..rows_on_universe {
            for col in 0..cells_on_row {
                let number_of_mines = self.number_of_bomb_neighbours(row as usize, col as usize);
                self.cells[col as usize][row as usize].set_neighboured_mines(number_of_mines)
            }
        }

        self.draw_cells();
        ;
    }

    pub fn init_mouse_events(&self) {
        let cb = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.offset_x();
            let y = event.offset_y();
            // let cell = web_sys::console::log_1(&p.into());
        }) as Box<dyn FnMut(_)>);

        self.canvas.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());

        cb.forget();
    }

    pub fn draw_cells(&self) -> () {
        for row in &self.cells {
            for cell in row {
                cell.draw_cell(&self.context);
            }
        }
        ;
    }

    fn number_of_bomb_neighbours(&self, row: usize, col: usize) -> u32 {
        let up = self.get_cell(row - 1, col);
        let up_right = self.get_cell(row - 1, col + 1);
        let right = self.get_cell(row, col + 1);
        let down_right = self.get_cell(row + 1, col + 1);
        let down = self.get_cell(row + 1, col);
        let down_left = self.get_cell(row + 1, col - 1);
        let left = self.get_cell(row, col - 1);
        let up_left = self.get_cell(row - 1, col - 1);

        let neighbours = vec![up, up_right, up_left, down, down_left, down_right, right, left];

        neighbours.iter().filter(|x| {
            match x {
                Some(cell) => x.unwrap().is_bomb(),
                _ => false,
            }
        }).count() as u32
    }

    fn get_cell(&self, row: usize, col: usize) -> Option<&Cell> {
        if self.is_in_universe(row as i32, col as i32) {
            return Some(&self.cells[col][row]);
        }
        None
    }

    fn is_in_universe(&self, row: i32, col: i32) -> bool {
        col >= 0 && col < (self.height / CELL_SIZE) as i32 && row >= 0 && row < (self.width / CELL_SIZE) as i32
    }

    fn get_cell_from_mouse(&self){

    }
}


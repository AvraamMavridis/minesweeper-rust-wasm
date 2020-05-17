use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

use crate::cell::{Cell, CellState};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    context: web_sys::CanvasRenderingContext2d,
    canvas: web_sys::HtmlCanvasElement,
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

        let cells = (0..width * height)
            .map(|_| {
                Cell::new((0, 0), (1, 1), CellState::Empty)
            })
            .collect();

        Universe {
            width,
            height,
            context,
            canvas,
            cells,
        }
    }


    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn context(&self) -> web_sys::CanvasRenderingContext2d {
        self.context.clone()
    }

    pub fn fill_background(&self) -> Result<(), JsValue> {
        self.context.begin_path();
        self.context.rect(0.0, 0.0, self.width as f64, self.height as f64);
        self.context.set_fill_style(&JsValue::from("red"));
        self.context.fill();
        Ok(())
    }

    pub fn get_cells(&self) -> *const Cell {
         self.cells.as_ptr()
    }
}


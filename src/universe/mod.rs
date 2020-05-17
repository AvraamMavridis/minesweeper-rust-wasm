// extern crate rand;
// mod cell;

// use cell::Cell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Universe {
    width: u32,
    height: u32,
    // cells: Vec<Cell>,
    context: web_sys::CanvasRenderingContext2d,
    canvas: web_sys::HtmlCanvasElement,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {

    pub fn new(width: u32, height: u32) -> Universe {
        let width = width;
        let height = height;
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas").unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        // let cells = (0..width * height)
        //     .map(|_| {
        //         let y: f64 = rand::random::<f64>();
        //         if y > 0.5 {
        //             Cell::Black
        //         } else {
        //             Cell::White
        //         }
        //     })
        //     .collect();

        Universe {
            width,
            height,
            context,
            canvas,
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

    // pub fn cells(&self) -> *const Cell {
    //     self.cells.as_ptr()
    // }
}


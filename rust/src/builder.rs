use crate::utils;
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement};

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Builder {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    root_id: String
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_str(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log_str(&format_args!($($t)*).to_string()))
}

// fn using_a_macro() {
//     log!("Hello {}!", "world");
//     log!("Let's print some numbers...");
//     log!("1 + 3 = {}", 1 + 3);
// }

#[wasm_bindgen]
impl Builder {
    pub fn new() -> Builder {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        let root_id = String::from("root");

        Builder {
            width,
            height,
            cells,
            root_id,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn alert(&self, text: &str) {
        alert(&format!("Hello, {}!", text));
    }

    pub fn document(&self) -> Document {
        let window = web_sys::window().expect("no window exists");
        let document = window.document().expect("window should have a document");
        document
    }

    pub fn body(&self) -> HtmlElement {
        let document = self.document();
        let body = document.body().expect("document should have a body");
        body
    }

    pub fn set_root_by_id(&mut self, root_id: &str) {
        log!("Registering app at id: {root_id}!");
        self.root_id = String::from(root_id);
    }

    // pub fn create_div(&self, element_id: &str, text: &str) -> Result<(), JsValue> {
    //     log!("Registering app at id: {element_id}!");
    //     let document = self.document();
    //     let root = document.get_element_by_id(element_id).expect("Element id not found");

    //     let p = document.create_element("p")?;
    //     p.set_inner_html(text);
    //     root.append_child(&p)?;
    //     Ok(())
    // }

    pub fn create_div(&self, text: &str) -> Result<(), JsValue> {
        let document = self.document();
        let root = document.get_element_by_id(&self.root_id).expect("Element id not found");

        let p = document.create_element("p")?;
        p.set_inner_html(text);
        root.append_child(&p)?;
        Ok(())
    }
}

impl fmt::Display for Builder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
use serde::Serialize;
use std::time::Duration;
use tsumeshogi_solver::Backend;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Solver;

#[derive(Serialize)]
pub struct Result {
    pub error: Option<String>,
    pub moves: Vec<String>,
}

#[wasm_bindgen]
impl Solver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self
    }
    pub fn solve(&self, sfen: String) -> JsValue {
        let result =
            match tsumeshogi_solver::solve(&sfen, Backend::Yasai, Some(Duration::from_secs(5))) {
                Ok(moves) => Result { error: None, moves },
                Err(_) => Result {
                    error: Some(String::from("timeout")),
                    moves: Vec::new(),
                },
            };
        JsValue::from_serde(&result).unwrap()
    }
}

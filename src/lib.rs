use serde::Serialize;
use std::time::Duration;
use tsumeshogi_solver::Backend;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize)]
pub struct Result {
    pub error: Option<String>,
    pub moves: Vec<String>,
}

#[wasm_bindgen]
pub fn solve(sfen: String) -> JsValue {
    let result = match tsumeshogi_solver::solve(&sfen, Backend::Yasai, Some(Duration::from_secs(5)))
    {
        Ok(moves) => Result { error: None, moves },
        Err(_) => Result {
            error: Some(String::from("timeout")),
            moves: Vec::new(),
        },
    };
    JsValue::from_serde(&result).unwrap()
}

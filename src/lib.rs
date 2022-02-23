use js_sys::Array;
use shogi::bitboard::Factory;
use shogi::Position;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Solver;

#[wasm_bindgen]
impl Solver {
    pub fn new() -> Self {
        Factory::init();
        console_error_panic_hook::set_once();
        Self
    }
    pub fn solve(&self, sfen: String) -> JsValue {
        let mut pos = Position::new();
        pos.set_sfen(&sfen).expect("failed to set sfen");
        JsValue::from(
            tsumeshogi_solver::solve(pos)
                .iter()
                .map(|m| JsValue::from_str(&m.to_string()))
                .collect::<Array>(),
        )
    }
}

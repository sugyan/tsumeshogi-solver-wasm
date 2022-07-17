use serde::Serialize;
use shogi_core::{PartialPosition, ToUsi};
use shogi_official_kifu::display_single_move_kansuji;
use shogi_usi_parser::FromUsi;
use solver::implementations::{HashMapTable, YasaiPosition};
use std::time::Duration;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Default)]
pub struct Moves {
    pub sfen: Vec<String>,
    pub kifu: Vec<String>,
}

#[derive(Serialize)]
pub struct Result {
    pub error: Option<String>,
    pub moves: Moves,
}

#[wasm_bindgen]
pub fn solve(sfen: String) -> JsValue {
    let pos = PartialPosition::from_usi(&format!("sfen {sfen}")).expect("failed to parse sfen");
    let result = match solver::solve::<YasaiPosition, HashMapTable>(
        pos.clone(),
        Some(Duration::from_secs(5)),
    ) {
        Ok(moves) => Result {
            error: None,
            moves: Moves {
                sfen: moves.iter().map(|m| m.to_usi_owned()).collect(),
                kifu: moves
                    .iter()
                    .scan(pos, |pos, &m| {
                        let ret = display_single_move_kansuji(pos, m);
                        pos.make_move(m);
                        ret
                    })
                    .collect(),
            },
        },
        Err(_) => Result {
            error: Some(String::from("timeout")),
            moves: Moves::default(),
        },
    };
    JsValue::from_serde(&result).unwrap()
}

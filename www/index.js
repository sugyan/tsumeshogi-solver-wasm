import { Solver } from "tsumeshogi-solver-wasm";

const solver = Solver.new();

window.addEventListener("DOMContentLoaded", () => {
  let sfen = "4k4/9/9/9/9/9/9/9/9 b 2r2b4g4s4n4l18p 1";
  document.getElementById("shogi-player").addEventListener("update", (e) => {
    sfen = e.detail.sfen;
  });
  document.getElementById("button").addEventListener("click", () => {
    document.getElementById("result").innerText = JSON.stringify(solver.solve(sfen))
  })
});

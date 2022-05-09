importScripts("./pkg/tsumeshogi_solver_wasm.js");
wasm_bindgen("./pkg/tsumeshogi_solver_wasm_bg.wasm").then(() => {
  this.addEventListener("message", (ev) => {
    const now = performance.now();
    const result = wasm_bindgen.solve(ev.data);
    const elapsed = `${(performance.now() - now).toFixed(3)}ms`;
    this.postMessage({ result, elapsed });
  })
});

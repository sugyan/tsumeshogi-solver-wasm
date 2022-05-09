const worker = new Worker("worker.js");
const solve = async (sfen) => {
  return new Promise((resolve) => {
    worker.addEventListener("message", (ev) => {
      resolve(ev.data);
    }, { once: true });
    worker.postMessage(sfen)
  });
}

window.addEventListener("DOMContentLoaded", () => {
  const player = document.getElementById("shogi-player");
  const params = new URLSearchParams(location.search);
  let sfen = params.get("sfen") || "4k4/9/9/9/9/9/9/9/9 b 2r2b4g4s4n4l18p 1";
  player.setAttribute("sfen", sfen);
  player.addEventListener("update", (e) => {
    sfen = e.detail.sfen;
    window.history.replaceState(null, null, `?${new URLSearchParams({ sfen }).toString()}`)
  });
  document.getElementById("button").addEventListener("click", () => {
    const button = document.getElementById("button");
    const output = document.getElementById("output");
    button.disabled = true;
    output.innerText = "searching...";
    solve(sfen).then((result) => {
      output.innerText = JSON.stringify(result, null, "  ")
    }).finally(() => {
      button.disabled = false;
    });
  })
});

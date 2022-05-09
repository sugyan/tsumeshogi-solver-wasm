(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("const worker = new Worker(\"worker.js\");\nconst solve = async (sfen) => {\n  return new Promise((resolve) => {\n    worker.addEventListener(\"message\", (ev) => {\n      resolve(ev.data);\n    }, { once: true });\n    worker.postMessage(sfen)\n  });\n}\n\nwindow.addEventListener(\"DOMContentLoaded\", () => {\n  const player = document.getElementById(\"shogi-player\");\n  const params = new URLSearchParams(location.search);\n  let sfen = params.get(\"sfen\") || \"4k4/9/9/9/9/9/9/9/9 b 2r2b4g4s4n4l18p 1\";\n  player.setAttribute(\"sfen\", sfen);\n  player.addEventListener(\"update\", (e) => {\n    sfen = e.detail.sfen;\n    window.history.replaceState(null, null, `?${new URLSearchParams({ sfen }).toString()}`)\n  });\n  document.getElementById(\"button\").addEventListener(\"click\", () => {\n    const button = document.getElementById(\"button\");\n    const output = document.getElementById(\"output\");\n    button.disabled = true;\n    output.innerText = \"searching...\";\n    solve(sfen).then((result) => {\n      output.innerText = JSON.stringify(result, null, \"  \")\n    }).finally(() => {\n      button.disabled = false;\n    });\n  })\n});\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);
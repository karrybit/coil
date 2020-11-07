let pkg = import("./pkg").catch(console.error);
let WASM = undefined;
let buf = undefined;

window.transition = async function transition() {
  if (!WASM) {
    WASM = await pkg;
  }
  if (!buf) {
    const image = await fetch('https://upload.wikimedia.org/wikipedia/commons/thumb/9/99/Unofficial_JavaScript_logo_2.svg/480px-Unofficial_JavaScript_logo_2.svg.png');
    buf = new Uint8Array(await (await image.blob()).arrayBuffer());
  }
  WASM.transition(buf);
};

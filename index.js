let pkg = import("./pkg").catch(console.error);
let WASM = undefined;
let jsBuf = undefined;
let wasmBuf = undefined;
let count = 0;

window.transition = async function transition() {
  if (!WASM) {
    WASM = await pkg;
  }
  let buf = undefined;
  if (count % 2 == 0) {
    if (!jsBuf) {
      const image = await fetch('https://upload.wikimedia.org/wikipedia/commons/thumb/9/99/Unofficial_JavaScript_logo_2.svg/480px-Unofficial_JavaScript_logo_2.svg.png');
      jsBuf = new Uint8Array(await (await image.blob()).arrayBuffer());
    }
    buf = jsBuf;
  } else {
    if (!wasmBuf) {
      const image = await fetch('https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/WebAssembly_Logo.svg/180px-WebAssembly_Logo.svg.png');
      wasmBuf = new Uint8Array(await (await image.blob()).arrayBuffer());
    }
    buf = wasmBuf;
  }
  WASM.transition(buf, false);
  count++;
};

window.revTransition = async function revTransition() {
  if (!WASM) {
    WASM = await pkg;
  }
  let buf = undefined;
  if (count % 2 == 0) {
    if (!jsBuf) {
      const image = await fetch('https://upload.wikimedia.org/wikipedia/commons/thumb/9/99/Unofficial_JavaScript_logo_2.svg/480px-Unofficial_JavaScript_logo_2.svg.png');
      jsBuf = new Uint8Array(await (await image.blob()).arrayBuffer());
    }
    buf = jsBuf;
  } else {
    if (!wasmBuf) {
      const image = await fetch('https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/WebAssembly_Logo.svg/180px-WebAssembly_Logo.svg.png');
      wasmBuf = new Uint8Array(await (await image.blob()).arrayBuffer());
    }
    buf = wasmBuf;
  }
  WASM.transition(buf, true);
  count++;
};

window.pager = async function pager() {
  if (!WASM) {
    WASM = await pkg;
  }
  if (!jsBuf) {
    const image = await fetch('https://upload.wikimedia.org/wikipedia/commons/thumb/9/99/Unofficial_JavaScript_logo_2.svg/480px-Unofficial_JavaScript_logo_2.svg.png');
    jsBuf = new Uint8Array(await (await image.blob()).arrayBuffer());
  }
  const buf = jsBuf;
  const { Pager } = WASM
  const pager = new Pager(buf, false);
  console.log(pager);
  pager.log();
}

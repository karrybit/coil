let pkg = import("./pkg").catch(console.error);
let WASM = undefined;
const imageURL1 = 'https://upload.wikimedia.org/wikipedia/commons/thumb/9/99/Unofficial_JavaScript_logo_2.svg/480px-Unofficial_JavaScript_logo_2.svg.png';
let imageBuf1 = undefined;
const imageURL2 = 'https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/WebAssembly_Logo.svg/480px-WebAssembly_Logo.svg.png';
let imageBuf2 = undefined;
let count = 0;

window.transition = async function transition() {
  if (!WASM) {
    WASM = await pkg;
  }
  let buf = undefined;
  if (count % 2 == 0) {
    if (!imageBuf1) {
      const image = await fetch(imageURL1);
      imageBuf1 = new Uint8Array(await (await image.blob()).arrayBuffer());
    }
    buf = imageBuf1;
  } else {
    if (!imageBuf2) {
      const image = await fetch(imageURL2);
      imageBuf2 = new Uint8Array(await (await image.blob()).arrayBuffer());
    }
    buf = imageBuf2;
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
    if (!imageBuf1) {
      const image = await fetch(imageURL1);
      imageBuf1 = new Uint8Array(await (await image.blob()).arrayBuffer());
    }
    buf = imageBuf1;
  } else {
    if (!imageBuf2) {
      const image = await fetch(imageURL2);
      imageBuf2 = new Uint8Array(await (await image.blob()).arrayBuffer());
    }
    buf = imageBuf2;
  }
  WASM.transition(buf, true);
  count++;
};

window.pager = async function pager() {
  if (!WASM) {
    WASM = await pkg;
  }
  if (!imageBuf1) {
    const image = await fetch(imageURL1);
    imageBuf1 = new Uint8Array(await (await image.blob()).arrayBuffer());
  }
  const buf = imageBuf1;
  const { Pager } = WASM
  const pager = new Pager(buf, false);
  console.log(pager);
  pager.log();
}

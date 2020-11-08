let pkg = import("./pkg").catch(console.error);
let WASM = undefined;
const imageURL1 = 'https://upload.wikimedia.org/wikipedia/commons/thumb/9/99/Unofficial_JavaScript_logo_2.svg/480px-Unofficial_JavaScript_logo_2.svg.png';
let imageBuf1 = undefined;
const imageURL2 = 'https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/WebAssembly_Logo.svg/480px-WebAssembly_Logo.svg.png';
let imageBuf2 = undefined;
let count = 0;

const load = async () => {
  if (!WASM) {
    WASM = await pkg;
  }
  if (!imageBuf1) {
    const image = await fetch(imageURL1);
    imageBuf1 = new Uint8Array(await (await image.blob()).arrayBuffer());
  }
  if (!imageBuf2) {
    const image = await fetch(imageURL2);
    imageBuf2 = new Uint8Array(await (await image.blob()).arrayBuffer());
  }
}

window.transition = async function transition() {
  await load();
  let before = count % 2 == 0 ? imageBuf1 : imageBuf2;
  let after = count % 2 != 0 ? imageBuf1 : imageBuf2;
  WASM.transition(before, after);
  count++;
};

window.revTransition = async function revTransition() {
  await load();
  let before = count % 2 == 0 ? imageBuf1 : imageBuf2;
  let after = count % 2 != 0 ? imageBuf1 : imageBuf2;
  WASM.transition(before, after);
  count++;
};

window.pager = async function pager() {
  await load();
  let before = count % 2 == 0 ? imageBuf1 : imageBuf2;
  let after = count % 2 != 0 ? imageBuf1 : imageBuf2;
  const { Pager } = WASM
  new Pager();
  WASM.transition(before, after);
  count++;
}

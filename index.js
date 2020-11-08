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

const INTERVAL = 12;

window.up = async function up() {
  await load();
  let before = count % 2 == 0 ? imageBuf1 : imageBuf2;
  let after = count % 2 != 0 ? imageBuf1 : imageBuf2;
  WASM.Pager.initialize();
  WASM.Pager.up(INTERVAL, before, after);
  count++;
};

window.right = async function right() {
  await load();
  let before = count % 2 == 0 ? imageBuf1 : imageBuf2;
  let after = count % 2 != 0 ? imageBuf1 : imageBuf2;
  WASM.Pager.initialize();
  WASM.Pager.right(INTERVAL, before, after);
  count++;
};

window.down = async function down() {
  await load();
  let before = count % 2 == 0 ? imageBuf1 : imageBuf2;
  let after = count % 2 != 0 ? imageBuf1 : imageBuf2;
  WASM.Pager.initialize();
  WASM.Pager.down(INTERVAL, before, after);
  count++;
};

window.left = async function left() {
  await load();
  let before = count % 2 == 0 ? imageBuf1 : imageBuf2;
  let after = count % 2 != 0 ? imageBuf1 : imageBuf2;
  WASM.Pager.initialize();
  WASM.Pager.left(INTERVAL, before, after);
  count++;
};

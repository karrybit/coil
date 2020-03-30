// For more comments about what's going on here, check out the `hello_world`
// example.
import("./pkg").catch(console.error);

window.transition = async function transition() {
  const WASM = await import("./pkg");
  WASM.transition();
};

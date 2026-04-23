/**
 * ffi-napi pilot — symmetry + deterministic hash for an existing `.aionai` file.
 *
 *   cargo build -p aion-engine --features ffi
 *   npm install
 *   node index.js path/to/capsule.aionai
 *
 * Create a capsule first, e.g. `cargo run -p aion-cli -- execute ai --model demo --prompt hi --seed 1`
 * then pass `aion_output/.../capsule.aionai`.
 */
const path = require("path");
const ffi = require("ffi-napi");
const ref = require("ref-napi");

const root = path.join(__dirname, "..", "..");
const libPath =
  process.platform === "win32"
    ? path.join(root, "target", "debug", "aion_engine.dll")
    : path.join(root, "target", "debug", `libaion_engine.${process.platform === "darwin" ? "dylib" : "so"}`);

const api = ffi.Library(libPath, {
  aion_replay_symmetry_ok: ["int", ["string", ref.refType("uint8")]],
  aion_capsule_deterministic_hash_hex: ["int", ["string", ref.refType("pointer")]],
  aion_free_string: ["void", ["pointer"]],
});

const capsule = process.argv[2];
if (!capsule) {
  console.error("usage: node index.js <capsule.aionai>");
  process.exit(2);
}

const sym = ref.alloc("uint8");
const rc1 = api.aion_replay_symmetry_ok(capsule, sym);
if (rc1 !== 0) {
  console.error("aion_replay_symmetry_ok failed", rc1);
  process.exit(1);
}

const phex = ref.alloc("pointer");
const rc2 = api.aion_capsule_deterministic_hash_hex(capsule, phex);
if (rc2 !== 0) {
  console.error("aion_capsule_deterministic_hash_hex failed", rc2);
  process.exit(1);
}
const addr = phex.deref();
let hex = "";
if (!addr.isNull()) {
  hex = addr.readCString();
  api.aion_free_string(addr);
}

console.log(
  JSON.stringify({
    product: "aion-os",
    replay_symmetry_ok: sym.deref() === 1,
    deterministic_hash_hex: hex,
  })
);

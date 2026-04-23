const ffi = require("ffi-napi");

const lib = ffi.Library("libaion", {
  aion_last_error: ["string", []],
  aion_telemetry_set_enabled: ["int", ["uchar"]],
});

module.exports = { lib };

// Engine layer.
//
// This is the *only* place allowed to reach sideways into the COS kernel.
// Today it exposes a trait + stub so the rest of the code has something
// concrete to link against; tomorrow a real adapter will replace the stub
// without any call-site changes in `core/` or `cli/`.

pub mod cos_adapter;

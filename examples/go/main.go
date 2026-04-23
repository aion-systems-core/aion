// Pilot cgo sample: capsule save → replay → symmetry → hash.
// Build engine first: cargo build -p aion-engine --features ffi
// Then: cd examples/go && set CGO_LDFLAGS=-L../../target/debug (Windows) && go run .

package main

/*
#cgo windows LDFLAGS: -L${SRCDIR}/../../target/debug -laion_engine
#cgo linux LDFLAGS: -L${SRCDIR}/../../target/debug -laion_engine
#cgo darwin LDFLAGS: -L${SRCDIR}/../../target/debug -laion_engine

#include <stdlib.h>
#include <stdint.h>

typedef struct {
	char* stdout_data;
	size_t stdout_len;
	char* stderr_data;
	size_t stderr_len;
	int exit_code;
	uint64_t duration_ms;
	char* capsule_id;
} AionRunResult;

typedef struct {
	char* model;
	char* prompt;
	uint64_t seed;
	char* determinism_profile_json;
	char* token_trace_json;
	char* events_json;
	char* graph_json;
	char* why_report_json;
	char* drift_report_json;
	char* evidence_path;
} AionCapsule;

int aion_capsule_save(const AionCapsule* capsule, const char* path);
int aion_replay_capsule(const char* path, AionRunResult* out);
int aion_replay_symmetry_ok(const char* path, unsigned char* out_ok);
int aion_capsule_deterministic_hash_hex(const char* path, char** out_hex);
void aion_free_string(char* s);
void aion_free_run_result(AionRunResult* r);
const char* aion_last_error(void);
*/
import "C"

import (
	"fmt"
	"os"
	"path/filepath"
	"unsafe"
)

func main() {
	tmp := filepath.Join(os.TempDir(), "aion_go_capsule.aionai")
	_ = os.Remove(tmp)

	model := C.CString("demo")
	prompt := C.CString("go cgo pilot")
	det := C.CString("{}")
	tt := C.CString("[]")
	ev := C.CString("[]")
	g := C.CString("{}")
	w := C.CString("{}")
	dr := C.CString("{}")
	evp := C.CString("")
	cap := C.AionCapsule{
		model:                    model,
		prompt:                   prompt,
		seed:                     11,
		determinism_profile_json: det,
		token_trace_json:         tt,
		events_json:              ev,
		graph_json:               g,
		why_report_json:          w,
		drift_report_json:        dr,
		evidence_path:            evp,
	}
	p := C.CString(tmp)
	rc := C.aion_capsule_save(&cap, p)
	if rc != 0 {
		fmt.Fprintln(os.Stderr, "aion_capsule_save", rc, C.GoString(C.aion_last_error()))
		os.Exit(1)
	}
	var rep C.AionRunResult
	rc = C.aion_replay_capsule(p, &rep)
	if rc != 0 {
		fmt.Fprintln(os.Stderr, "aion_replay_capsule", rc, C.GoString(C.aion_last_error()))
		os.Exit(1)
	}
	var sym C.uchar
	_ = C.aion_replay_symmetry_ok(p, &sym)
	var hx *C.char
	_ = C.aion_capsule_deterministic_hash_hex(p, &hx)
	fmt.Println("AION | go pilot — replay exit", rep.exit_code, "symmetry_ok", sym == 1)
	if hx != nil {
		fmt.Println("AION | deterministic_hash_hex", C.GoString(hx))
		C.aion_free_string(hx)
	}
	C.aion_free_run_result(&rep)
	C.free(unsafe.Pointer(p))
	C.free(unsafe.Pointer(model))
	C.free(unsafe.Pointer(prompt))
	C.free(unsafe.Pointer(det))
	C.free(unsafe.Pointer(tt))
	C.free(unsafe.Pointer(ev))
	C.free(unsafe.Pointer(g))
	C.free(unsafe.Pointer(w))
	C.free(unsafe.Pointer(dr))
	C.free(unsafe.Pointer(evp))
}

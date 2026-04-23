package main

/*
#cgo CFLAGS: -I${SRCDIR}/../../../include
#cgo LDFLAGS: -L${SRCDIR}/../../../target/debug -laion_engine
#include <aion/aion.h>
#include <stdlib.h>
*/
import "C"
import (
	"fmt"
	"os"
	"unsafe"
)

func check(code C.int32_t) {
	if code != C.AION_OK {
		e := C.aion_last_error()
		msg := ""
		if e != nil {
			msg = C.GoString(e)
		}
		fmt.Fprintf(os.Stderr, "ERR code=%d msg=%s\n", int32(code), msg)
		os.Exit(1)
	}
}

func main() {
	out := C.CString("showcase_capsule_go.aionai")
	defer C.free(unsafe.Pointer(out))
	var cap C.AionCapsule
	m := C.CString("demo")
	p := C.CString("hello")
	defer C.free(unsafe.Pointer(m))
	defer C.free(unsafe.Pointer(p))
	cap.model = m
	cap.prompt = p
	cap.seed = 9
	check(C.aion_capsule_save(&cap, out))
	fmt.Println("capsule_save ok")

	var rr C.AionRunResult
	check(C.aion_replay_capsule(out, &rr))
	fmt.Printf("replay ok exit=%d\n", int32(rr.exit_code))
	C.aion_free_run_result(&rr)
}

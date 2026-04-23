package aion

/*
#cgo LDFLAGS: -laion
#include "../../include/aion/aion.h"
*/
import "C"
import "errors"

func check(code C.int32_t) error {
	if code == 0 {
		return nil
	}
	return errors.New(C.GoString(C.aion_last_error()))
}

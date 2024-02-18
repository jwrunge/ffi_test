package main

import "C"

func main() {}

//export HelloGo
func HelloGo(name *C.char) *C.char {
	return C.CString("Hello, World!")
}

package main

import "C"

func main() {}

//export HelloGo
func HelloGo() *C.char {
	return C.CString("Hello, World!")
}

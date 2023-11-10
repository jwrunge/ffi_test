package main

import "C"

//export hello_world
func hello_world() *C.char {
	return C.CString("Hello, World!")
}

func main() {}

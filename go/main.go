package main

import "C"

func main() {}

//export Hello
func Hello() *C.char {
	return C.CString("Hello, World!")
}

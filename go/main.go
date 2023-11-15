package main

import "C"
import (
	"fmt"
)

var html string

//export htmlPtr
func htmlPtr() *string {
	return &html
}

//export add
func add(a, b int) int {
	fmt.Println("Hello from Go!")
	return a + b
}

//export retStr
func retStr() string {
	return "Passed string response!"
}

func main() {
	fmt.Println("Main")
}

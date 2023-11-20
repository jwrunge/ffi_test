package main

var output string = "Hello from TinyGo!"

//export output_ptr
func output_ptr() string {
	return output
}

//export output_len
func output_len() int {
	return len(output)
}

func main() {}

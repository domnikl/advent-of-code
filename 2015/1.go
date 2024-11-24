package main

import (
	"fmt"
	"os"
)

func main() {
	input, err := os.ReadFile("1.input.txt")

	if err != nil {
		panic("Unable to read input")
	}

	floor := 0

	for _, x := range string(input) {
		if string(x) == "(" {
			floor++
		} else {
			floor--
		}
	}

	fmt.Println(floor)
}

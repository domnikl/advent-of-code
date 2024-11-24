package main

import (
	"fmt"
	"os"
)

func calc(input string) (int, int) {
	floor := 0
	var cellar int

	for i, x := range input {
		if string(x) == "(" {
			floor++
		} else {
			floor--
		}

		if floor < 0 && cellar == 0 {
			cellar = i + 1
		}
	}

	return floor, cellar
}

func main() {
	input, err := os.ReadFile("1.input.txt")

	if err != nil {
		panic("Unable to read input")
	}

	floor, cellar := calc(string(input))

	fmt.Println(floor)
	fmt.Println(cellar)
}

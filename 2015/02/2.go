package main

import (
	"bufio"
	"fmt"
	"os"
)

func calc(scanner *bufio.Scanner) int {
	totalSize := 0
	var l int
	var w int
	var h int

	for scanner.Scan() {
		fmt.Sscanf(scanner.Text(), "%dx%dx%d", &l, &w, &h)

		sideA := l * w
		sideB := w * h
		sideC := h * l

		totalSize += 2*sideA + 2*sideB + 2*sideC

		if sideA <= sideB && sideA <= sideC {
			totalSize += sideA
		} else if sideB <= sideC && sideB <= sideA {
			totalSize += sideB
		} else {
			totalSize += sideC
		}
	}

	return totalSize
}

func main() {
	input, err := os.Open("2.input.txt")

	if err != nil {
		panic("Unable to read input")
	}

	defer input.Close()

	totalSize := calc(bufio.NewScanner(input))

	fmt.Println(totalSize)
}

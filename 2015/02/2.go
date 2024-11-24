package main

import (
	"bufio"
	"fmt"
	"os"
)

func calc(scanner *bufio.Scanner) (int, int) {
	wrappingPaper := 0
	ribbonLength := 0
	var l int
	var w int
	var h int

	for scanner.Scan() {
		fmt.Sscanf(scanner.Text(), "%dx%dx%d", &l, &w, &h)

		sideA := l * w
		sideB := w * h
		sideC := h * l

		wrappingPaper += 2*sideA + 2*sideB + 2*sideC

		if sideA <= sideB && sideA <= sideC { // sideA is the smallest
			wrappingPaper += sideA
			ribbonLength += 2*l + 2*w
		} else if sideB <= sideC && sideB <= sideA { // sideB is the smallest
			wrappingPaper += sideB
			ribbonLength += 2*w + 2*h
		} else { // sideC is the smallest
			wrappingPaper += sideC
			ribbonLength += 2*h + 2*l
		}

		ribbonLength += l * w * h
	}

	return wrappingPaper, ribbonLength
}

func main() {
	input, err := os.Open("2.input.txt")

	if err != nil {
		panic("Unable to read input")
	}

	defer input.Close()

	wrappingPaper, ribbon := calc(bufio.NewScanner(input))

	fmt.Println(wrappingPaper)
	fmt.Println(ribbon)
}

package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	input, err := os.Open("3.input.txt")

	if err != nil {
		panic("Unable to read input")
	}

	houses := make(map[[2]int]int)

	defer input.Close()

	housesVisited := 0
	x, y := 0, 0
	scanner := bufio.NewScanner(input)

	for scanner.Scan() {
		for _, r := range scanner.Text() {
			if r == '^' {
				y++
			} else if r == 'v' {
				y--
			} else if r == '<' {
				x--
			} else if r == '>' {
				x++
			}

			houses[[2]int{x, y}]++
		}
	}

	for _, v := range houses {
		if v > 0 {
			housesVisited++
		}
	}

	fmt.Println(housesVisited)
}

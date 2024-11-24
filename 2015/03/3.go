package main

import (
	"bufio"
	"fmt"
	"os"
)

func calc(scanner *bufio.Scanner, giftersCount int) int {
	housesVisited := 0
	houses := make(map[[2]int]int)
	gifterPositions := make([][2]int, giftersCount)
	currentGifter := 0

	for scanner.Scan() {
		for _, r := range scanner.Text() {
			if r == '^' {
				gifterPositions[currentGifter][1]++
			} else if r == 'v' {
				gifterPositions[currentGifter][1]--
			} else if r == '<' {
				gifterPositions[currentGifter][0]--
			} else if r == '>' {
				gifterPositions[currentGifter][0]++
			}

			houses[[2]int{gifterPositions[currentGifter][0], gifterPositions[currentGifter][1]}]++

			if currentGifter+1 == giftersCount {
				currentGifter = 0
			} else {
				currentGifter++
			}
		}
	}

	for _, v := range houses {
		if v > 0 {
			housesVisited++
		}
	}

	return housesVisited
}

func main() {
	input, err := os.Open("3.input.txt")

	if err != nil {
		panic("Unable to read input")
	}

	defer input.Close()

	housesVisited := calc(bufio.NewScanner(input), 1)
	input.Seek(0, 0)
	presentsGiven := calc(bufio.NewScanner(input), 2)

	fmt.Println(housesVisited)
	fmt.Println(presentsGiven)
}

package main

import (
	"bufio"
	"math"
	"os"
	"sort"
	"strconv"
)

func getInput() *os.File {
	input, err := os.Open("1.input.txt")

	if err != nil {
		panic("Unable to read input")
	}

	return input
}

func main() {
	input := getInput()

	defer input.Close()

	scanner := bufio.NewScanner(input)
	scanner.Split(bufio.ScanWords)

	listA := make([]int, 0)
	listB := make([]int, 0)
	i := 0

	for scanner.Scan() {
		value, err := strconv.Atoi(scanner.Text())

		if err != nil {
			panic("Unable to convert to int")
		}

		if i%2 == 0 {
			listA = append(listA, value)
		} else {
			listB = append(listB, value)
		}

		i++
	}

	// sort both lists
	sort.Sort(sort.IntSlice(listA))
	sort.Sort(sort.IntSlice(listB))

	total := 0.

	for i := 0; i < len(listA); i++ {
		a := listA[i]
		b := listB[i]

		total += math.Abs(float64(a - b))
	}

	println(int(total))
}

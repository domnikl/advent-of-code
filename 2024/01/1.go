package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
)

func getInput() ([]int, []int) {
	input, err := os.Open("1.input.txt")

	if err != nil {
		panic("Unable to read input")
	}

	defer input.Close()

	scanner := bufio.NewScanner(input)
	scanner.Split(bufio.ScanWords)

	left := make([]int, 0)
	right := make([]int, 0)
	i := 0

	for scanner.Scan() {
		value, err := strconv.Atoi(scanner.Text())

		if err != nil {
			panic("Unable to convert to int")
		}

		if i%2 == 0 {
			left = append(left, value)
		} else {
			right = append(right, value)
		}

		i++
	}

	return left, right
}

func part1(left, right []int) int {
	sort.Sort(sort.IntSlice(left))
	sort.Sort(sort.IntSlice(right))

	total := 0.

	for i := 0; i < len(left); i++ {
		a := left[i]
		b := right[i]

		total += math.Abs(float64(a - b))
	}

	return int(total)
}

func part2(left, right []int) int {
	sum := 0

	for i := 0; i < len(left); i++ {
		for j := 0; j < len(right); j++ {
			if left[i] == right[j] {
				sum += left[i]
			}
		}
	}

	return sum
}

func main() {
	left, right := getInput()

	fmt.Println(part1(left, right))
	fmt.Println(part2(left, right))
}

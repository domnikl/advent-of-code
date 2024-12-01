package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
)

func GetInput() ([]int, []int) {
	input, err := os.Open("1.input.txt")

	if err != nil {
		panic("Unable to read input")
	}

	defer input.Close()

	scanner := bufio.NewScanner(input)
	scanner.Split(bufio.ScanWords)

	left := make([]int, 1000)
	right := make([]int, 1000)
	var i uint

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

func Solve(left, right []int) (int, int, error) {
	sort.Sort(sort.IntSlice(left))
	sort.Sort(sort.IntSlice(right))

	var part1, part2 int

	for i, l := range left {
		// Part 1
		part1 += int(math.Abs(float64(l - right[i])))

		// Part 2
		for j := 0; j < len(right); j++ {
			if l == right[j] {
				part2 += l
			}
		}
	}

	return part1, part2, nil
}

func main() {
	left, right := GetInput()
	part1, part2, err := Solve(left, right)

	if err != nil {
		panic("Unable to solve")
	}

	fmt.Println(part1)
	fmt.Println(part2)
}

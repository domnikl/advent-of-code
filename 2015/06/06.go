package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func convertCoordinates(x1, x2, x3, x4 string) (int, int, int, int) {
	x1Int, _ := strconv.Atoi(x1)
	x2Int, _ := strconv.Atoi(x2)
	x3Int, _ := strconv.Atoi(x3)
	x4Int, _ := strconv.Atoi(x4)

	return x1Int, x2Int, x3Int, x4Int
}

func part1(grid *[][]bool) {
	file, err := os.Open("6.input.txt")

	if err != nil {
		panic("Unable to read input")
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		// Do something with line

		// Parse 'turn on 0,0 through 999,999'
		matches := regexp.MustCompile(`^(.*) (\d+),(\d+) through (\d+),(\d+)$`).FindStringSubmatch(line)
		mode := matches[1]
		x1, y1, x2, y2 := convertCoordinates(matches[2], matches[3], matches[4], matches[5])

		for i := x1; i <= x2; i++ {
			for j := y1; j <= y2; j++ {
				if mode == "turn off" {
					(*grid)[i][j] = false
				} else if mode == "turn on" {
					(*grid)[i][j] = true
				} else if mode == "toggle" {
					(*grid)[i][j] = !(*grid)[i][j]
				}
			}
		}
	}
}

func main() {
	gridSize := 1000
	grid := make([][]bool, gridSize)

	for i := 0; i < gridSize; i++ {
		grid[i] = make([]bool, gridSize)
	}

	part1(&grid)

	// find all values in grid that are true and count them
	total := 0

	for i := 0; i < 1000; i++ {
		for j := 0; j < 1000; j++ {
			if grid[i][j] {
				total++
			}
		}
	}

	fmt.Println(total)
}

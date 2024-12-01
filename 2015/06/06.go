package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

type command struct {
	mode string
	x1   int
	y1   int
	x2   int
	y2   int
}

func convertCoordinates(x1, x2, x3, x4 string) (int, int, int, int) {
	x1Int, _ := strconv.Atoi(x1)
	x2Int, _ := strconv.Atoi(x2)
	x3Int, _ := strconv.Atoi(x3)
	x4Int, _ := strconv.Atoi(x4)

	return x1Int, x2Int, x3Int, x4Int
}

func getCommands() []command {
	file, err := os.Open("6.input.txt")

	if err != nil {
		panic("Unable to read input")
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)
	commands := make([]command, 0)

	for scanner.Scan() {
		line := scanner.Text()

		// Do something with line

		// Parse 'turn on 0,0 through 999,999'
		matches := regexp.MustCompile(`^(.*) (\d+),(\d+) through (\d+),(\d+)$`).FindStringSubmatch(line)
		mode := matches[1]
		x1, y1, x2, y2 := convertCoordinates(matches[2], matches[3], matches[4], matches[5])

		commands = append(commands, command{mode, x1, y1, x2, y2})
	}

	return commands
}

func part1(commands *[]command) [][]bool {
	gridSize := 1000
	grid := make([][]bool, gridSize)

	for i := 0; i < gridSize; i++ {
		grid[i] = make([]bool, gridSize)
	}

	for _, c := range *commands {
		for i := c.x1; i <= c.x2; i++ {
			for j := c.y1; j <= c.y2; j++ {
				if c.mode == "turn off" {
					grid[i][j] = false
				} else if c.mode == "turn on" {
					grid[i][j] = true
				} else if c.mode == "toggle" {
					grid[i][j] = !grid[i][j]
				}
			}
		}
	}

	return grid
}

func part1_x(commands *[]command) int {
	grid := part1(commands)

	// find all values in grid that are true and count them
	total := 0

	for i := 0; i < 1000; i++ {
		for j := 0; j < 1000; j++ {
			if grid[i][j] {
				total++
			}
		}
	}

	return total
}

func part2(commands *[]command) [][]int {
	gridSize := 1000
	grid := make([][]int, gridSize)

	for i := 0; i < gridSize; i++ {
		grid[i] = make([]int, gridSize)
	}

	for _, c := range *commands {
		for i := c.x1; i <= c.x2; i++ {
			for j := c.y1; j <= c.y2; j++ {
				if c.mode == "turn off" {
					grid[i][j]--
				} else if c.mode == "turn on" {
					grid[i][j]++
				} else if c.mode == "toggle" {
					grid[i][j] += 2
				}

				if grid[i][j] < 0 {
					grid[i][j] = 0
				}
			}
		}
	}

	return grid
}

func part2_x(commands *[]command) int {
	grid := part2(commands)
	total := 0

	for i := 0; i < 1000; i++ {
		for j := 0; j < 1000; j++ {
			total += grid[i][j]
		}
	}

	return total
}

func main() {
	commands := getCommands()

	fmt.Println(part1_x(&commands))
	fmt.Println(part2_x(&commands))
}

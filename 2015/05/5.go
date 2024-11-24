package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func countVowels(s string) int {
	return strings.Count(s, "a") + strings.Count(s, "e") + strings.Count(s, "i") + strings.Count(s, "o") + strings.Count(s, "u")
}

func hasDoubleLetters(s string) bool {
	for i := 0; i < len(s)-1; i++ {
		if s[i] == s[i+1] {
			return true
		}
	}

	return false
}

func hasBadStrings(s string) bool {
	return strings.Contains(s, "ab") || strings.Contains(s, "cd") || strings.Contains(s, "pq") || strings.Contains(s, "xy")
}

func countNiceStrings(scanner *bufio.Scanner) int {
	niceStrings := 0

	for scanner.Scan() {
		line := scanner.Text()

		if countVowels(line) >= 3 && hasDoubleLetters(line) && !hasBadStrings(line) {
			niceStrings++
		}
	}

	return niceStrings
}

func hasDoublePair(s string) bool {
	for i := 0; i < len(s)-3; i++ {

		if strings.Contains(s[i+2:], s[i:i+2]) {
			return true
		}
	}

	return false
}

func hasRepeatingLetter(s string) bool {
	for i := 0; i < len(s)-2; i++ {
		if s[i] == s[i+2] {
			return true
		}
	}

	return false
}

func countNiceStringsNewRules(scanner *bufio.Scanner) int {
	niceStrings := 0

	for scanner.Scan() {
		line := scanner.Text()

		if hasDoublePair(line) && hasRepeatingLetter(line) {
			niceStrings++
		}
	}

	return niceStrings
}

func main() {
	input, err := os.Open("5.input.txt")

	if err != nil {
		panic("Unable to read input")
	}

	defer input.Close()

	fmt.Println(countNiceStrings(bufio.NewScanner(input)))
	input.Seek(0, 0)
	fmt.Println(countNiceStringsNewRules(bufio.NewScanner(input)))
}

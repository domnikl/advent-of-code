package main

import "testing"

func TestSolve(t *testing.T) {
	left := []int{3, 4, 2, 1, 3, 3}
	right := []int{4, 3, 5, 3, 9, 3}

	part1, part2, err := Solve(left, right)

	if err != nil {
		t.Fatalf("Test failed")
	}

	if part1 != 11 {
		t.Fatalf("Part 1 failed")
	}

	if part2 != 31 {
		t.Fatalf("Part 2 failed")
	}
}

func BenchmarkSolve(b *testing.B) {
	left, right := GetInput()

	// run the Fib function b.N times
	for n := 0; n < b.N; n++ {
		Solve(left, right)
	}
}

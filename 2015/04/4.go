package main

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"os"
)

func findHash(input []byte, prefix string) int {
	for i := 1; ; i++ {
		foo := md5.Sum([]byte(fmt.Sprintf("%s%d", input, i)))

		if hex.EncodeToString(foo[:])[:len(prefix)] == prefix {
			return i
		}
	}
}

func main() {
	input, err := os.ReadFile("4.input.txt")

	if err != nil {
		panic("Unable to read input")
	}

	fmt.Println(findHash(input, "00000"))
	fmt.Println(findHash(input, "000000"))
}

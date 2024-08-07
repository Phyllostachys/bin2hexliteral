package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	var filePath string
	var outputLength int
	flag.StringVar(&filePath, "file", ".", "file to convert from binary to hex literal")
	flag.IntVar(&outputLength, "length", 16, "amount of hex literals to output per line")
	flag.Parse()

	data, err := os.ReadFile(filePath)
	if err != nil {
		fmt.Println(err)
		return
	}

	for i := 0; i < len(data); i += outputLength {
		endOfLineIndex := i + outputLength
		for j := i; (j < endOfLineIndex) && (j < len(data)); j++ {
			fmt.Printf("0x%02x, ", data[j])
		}
		fmt.Println()
	}
}

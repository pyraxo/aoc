package main

import (
	"fmt"
	"bufio"
	"strconv"
	"time"
	"os"
)

func check (e error) {
	if e != nil {
		fmt.Println(e)
	}
}


func main () {
	start := time.Now()
	var sum int
	file, err := os.Open("./input.txt")
	check(err)
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		num, err := strconv.Atoi(line)
		check(err)
		sum += num
	}

	fmt.Println(sum)
	fmt.Printf("--- %v ---\n", time.Since(start))
}
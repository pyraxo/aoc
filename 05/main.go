package main

import (
	"fmt"
	"log"
	"time"
	"strings"
	"io/ioutil"
	"os"
)

func check (e error) {
	if e != nil {
		log.Panic(e)
	}
}

func main () {
	file, err := os.Open("./input.txt")
	check(err)
	defer file.Close()

	// part 1
	start := time.Now()

	origList := make([]byte, 50000)
	_, err = file.Read(origList)
	check(err)
	
	charList := append(origList[:0:0], origList...)
	result := react(charList)

	elapsed := time.Since(start)
	fmt.Printf("part 1: %v\n", result)
	fmt.Printf("--- %v ---\n", elapsed)

	// part 1 alternative, slower
	start = time.Now()
	origList, err = ioutil.ReadFile("./input.txt")
	check(err)

	var origStr string
	str := string(origList)
	best := 50000
	for str != origStr {
		origStr = str
		for i := 65; i < 91; i++ {
			toReplace1 := string(rune(i)) + string(rune(i + 32))
			toReplace2 := string(rune(i + 32)) + string(rune(i))
			str = strings.Replace(str, toReplace1, "", -1)
			str = strings.Replace(str, toReplace2, "", -1)
			currLen := len(str)
			if currLen < best {
				best = currLen
			}
		}
	}

	elapsed = time.Since(start)
	fmt.Printf("part 1 alt: %v\n", best)
	fmt.Printf("--- %v ---\n", elapsed)

	// part 2
	start = time.Now()

	best = 50000
	for i := 65; i < 91; i++ {
		newList := remove(append(origList[:0:0], origList...), i)
		result := react(newList)

		if result < best {
			best = result
		}
	}

	elapsed = time.Since(start)
	fmt.Printf("part 2: %v\n", best)
	fmt.Printf("--- %v ---\n", elapsed) 
}

func react (list []byte) (result int) {
	var count int
	for count + 1 != len(list) {
		count = 0
		for i := 1; i < len(list); i++ {
			if list[i] == list[i - 1] - 32 || list[i - 1] == list[i] - 32 {
				if len(list) == i + 1 {
					list = list[:i-1]
				} else {
					list = append(list[:i-1], list[i+1:]...)
				}
				break
			}
			count++
		}
	}
	result = len(list)
	return
}

func remove (list []byte, ascii int) (newList []byte) {
	newList = list
	for i := 0; i < len(newList); {
		if newList[i] == byte(ascii) || newList[i] == byte(ascii + 32) {
			newList = append(newList[:i], newList[i+1:]...)
			continue
		}
		i++
	}
	return
}
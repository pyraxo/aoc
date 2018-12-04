package main

import (
	"fmt"
	"time"
	"bufio"
	"strings"
	"strconv"
	"regexp"
	"sort"
	"log"
	"os"
)

func check (e error) {
	if e != nil {
		log.Fatal(e)
	}
}

type Entry struct {
	time time.Time
	id int
}

type Guard struct {
	mins map[int]int
	totalMin int
}

func main() {
	file, err := os.Open("./input.txt")
	check(err)
	defer file.Close()

	// part 1
	start := time.Now()
	layout := "2006-01-02 15:04"
	var store []Entry

	// Getting and sorting entries
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		has := strings.Contains
		line := scanner.Text()
		t, err := time.Parse(layout, line[1:17])
		check(err)

		var id int
		if has(line, "#") {
			re := regexp.MustCompile(`#(\d{2,4})`)
			id, _ = strconv.Atoi(re.FindStringSubmatch(line)[1])
			entry := Entry{t, id}
			store = append(store, entry)
		} else if has(line, "falls") {
			store = append(store, Entry{t, 1})
		} else if has(line, "wakes") {
			store = append(store, Entry{t, 2})
		}
	}

	sort.Slice(store, func (i, j int) bool {
		return store[i].time.Before(store[j].time)
	})

	sortStage := time.Since(start)

	// Parsing in order
	var guard, started int
	guards := map[int]*Guard{}
	for _, entry := range store {
		if entry.id == 1 {
			started = entry.time.Minute()
		} else if entry.id == 2 {
			ended := entry.time.Minute()
			elapsed := ended - started
			guards[guard].totalMin += elapsed
			for i := started; i < ended; i++ {
				if guards[guard].mins != nil {
					guards[guard].mins[i]++
				} else {
					guards[guard].mins = make(map[int]int)
				}
			}
		} else {
			guard = entry.id
			if guards[guard] == nil {
				guards[guard] = &Guard{}
			}
		}
	}
	parseStage := time.Since(start)

	// Finding highest guard
	var maxTotal, maxGuard int
	for id, guard := range guards {
		count := guard.totalMin
		if count > maxTotal {
			maxTotal = count
			maxGuard = id
		}
	}

	// Finding highest count
	var maxCount, maxMin int
	for min, count := range guards[maxGuard].mins {
		if count > maxCount {
			maxCount = count
			maxMin = min
		}
	}
	result := maxGuard * maxMin
	endStage := time.Since(start)

	fmt.Printf("sorting: %v\n", sortStage)
	fmt.Printf("parsing: %v\n", parseStage)
	fmt.Printf("part 1: %v\n", result)
	fmt.Printf("--- %v ---\n\n", endStage)

	// part 2
	start = time.Now()
	var highestMin, highestCount, highestGuard int
	for i := 0; i < 60; i++ {
		for id, guard := range guards {
			if guard.mins[i] > highestCount {
				highestGuard = id
				highestMin = i
				highestCount = guard.mins[i]
			}
		}
	}

	result2 := highestMin * highestGuard
	endStage2 := time.Since(start)
	fmt.Printf("part 2: %v\n", result2)
	fmt.Printf("--- %v ---\n\n", endStage2)
}
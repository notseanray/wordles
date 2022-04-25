package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"math/rand"
	"os"
	"strconv"
	"strings"
	"time"
)

var green = "\x1b[30m\x1b[42m"
var yellow = "\x1b[30m\x1b[43m"
var	reset = "\x1b[0m"
var clear = "\x1b[H\x1b[2J"

type Words struct {
	words []string
	used []string
}

func gather_data(path string) Words {
	contents, err := ioutil.ReadFile(path)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	word_list := Words{}
	err = json.Unmarshal(contents, &word_list.words)
	if err != nil {
		fmt.Println(err)
	}
	return word_list
}

func filter_words(word_list []string, target_length int) []string {
	var words []string
	for _, s := range word_list {
		if len(s) != target_length {
			continue
		}
		words = append(words, s)
	}
	return words
}

// does not need to be generic, but I wanted to see what generics are like in go
type Equality interface {
	string
}

func contains [T Equality](characters []T, character T) bool {
	for _, s := range characters {
		if s == character {
			return true
		}
	}
	return false
}

func print_board(board [][]string, target_word string) {
	fmt.Println(clear)
	characters := strings.Split(target_word, "")
	for _, r := range board {
		for i, c := range r {
			var color string
			if c == characters[i] {
				color = green
			} else if contains(characters, c) {
				color = yellow 
			}
			fmt.Print(color, " ", c, " ", reset)
		} 
		fmt.Println()
	}
}

func remove(list []string, target string) []string {
	target_index := 0
	for i, e := range list {
		if e == target {
			target_index = i
		}
	}
	return append(list[:target_index], list[target_index + 1:]...)
}

func update_wordlist(target_word string) {
	list := gather_data("./words.json").words
	list = remove(list, target_word)
	file, _ := json.MarshalIndent(list, "", " ")
	_ = ioutil.WriteFile("words.json", file, 0644)
	removed_list := gather_data("./used.json").words
	removed_list = append(removed_list, target_word)
	removed_file, _ := json.MarshalIndent(removed_list, "", " ")
	_ = ioutil.WriteFile("used.json", removed_file, 0644)
}

func add_guess(board [][]string, guess string, target_word string, start time.Time) [][]string {
	correct := guess == target_word
	guesses := 0
	characters := strings.Split(guess, "")
	for _, r := range board {
		if len(strings.TrimSpace(r[0])) != 0 {
			continue
		}
		guesses++
		for i, c := range characters {
			r[i] = c
		}
		if !correct {
			return board
		}
		break
	}
	// call exit here
	print_board(board, target_word)
	fmt.Println("elapsed: ", float32(int(time.Since(start)) / 10000000) / 100, "s, guesses: [", guesses, "/", len(characters), "]")
	update_wordlist(target_word)
	os.Exit(0)
	// this never actually returns but we must make the compiler shut up
	return board
}

func main() {
	start := time.Now()
	word_length := 5
	args := os.Args
	if len(args) > 1 {
		i, err := strconv.Atoi(args[1])
		if err != nil {
			fmt.Println("Please provide a proper length of word to play with, default is 5 if non is specified!")
			os.Exit(1)
		}
		word_length = i
	}
	json_data := gather_data("./words.json")
	word_list := filter_words(json_data.words, word_length)
	if len(word_list) == 1 {
		fmt.Println("No words of this length, please try a different length!")
		os.Exit(0)
	}
	selected_word := strings.ToLower(word_list[rand.Intn(len(word_list))])
	fmt.Println(clear)
	fmt.Println("Word Length: ", word_length, " Guesses: ", word_length)
	var board [][]string
	for h := 0; h < word_length; h++ {
		var row []string
		for w := 0; w < word_length; w++ {
			row = append(row, " ")
		}
		board = append(board, row)
	}
	for guess := 1; guess <= word_length; guess++ {
		var first_guess string
		for {
			_, _ = fmt.Scan(&first_guess)
			if len(first_guess) == word_length {
				break;
			}
			fmt.Println("invalid length")
		}
		board = add_guess(board, first_guess, selected_word, start)
		print_board(board, selected_word)
		fmt.Println("elapsed: ", float32(int(time.Since(start)) / 10000000) / 100, "s")
	}
	fmt.Println("better luck next time (:\nthe word was ", selected_word)
}

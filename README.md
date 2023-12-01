# Advent of code 2023

https://adventofcode.com/

This is my first real experience with Rust. I'm using it as a learning experience. Normally I would be more 
comfortable with Go, but I wanted to try something new that could be useful in the future.

Normally I would also refactor and clean up the code after I got the solution working, but I want to see
the evolution of my code as I learn more about Rust.

## Day1

Part one was pretty straight forward. I used basic for each loops to iterate over the input and then find the digits. No funky Rust feature required here.

Part two was more tricky, particularly because of some traps like: "eightwothree" where it cause issue with my indexes of zero previously found for part 1 that I use two know if my "word digit" is after or before the numerical digit.
# Advent of code 2023

https://adventofcode.com/

This is my first real experience with Rust. I'm using it as a learning experience. Normally I would be more 
comfortable with Go, but I wanted to try something new that could be useful in the future.

Normally I would also refactor and clean up the code after I got the solution working, but I want to see
the evolution of my code as I learn more about Rust.

## Day1

### Part 1
Pretty straight forward. I used basic for each loops to iterate over the input and then find the digits. No funky Rust feature required here.

### Part 2
Part two was more tricky, particularly because of some traps like: "eightwothree" where it cause issue with my indexes of zero previously found for part 1 that I use two know if my "word digit" is after or before the numerical digit.

## Day2

### Part 1
No real issue here, I cleaned the input string and split by whitespace the data so that the previous token is always
to number of cubes of the actual token.

### Part 2
Again, no challenge here in the problem itself, straight forward technique to find the maximum of each colors.

## Day3

### Part 1
I choose to go with a simple approach, I used a regex to find the numbers in each string and then boiler plate code to
find the neibourghs making sure to deal with the edges of the input. I don't know if there is a better way to deal with
the edges, my method is not very elegant.

I had a struggle with single digit numbers because I was using find() to get the index of the number in the string and if
the regex was matching a 9, it would also match the 9 in 19 and 29. Turn's out you can easily get the start and end of the
match

### Part 2
Sometimes with AoC, your first part helps you with the second part, sometimes it doesn't. In this case, it did not help
so I basically had to start over. But the logic was mostly the same: Find '*', look for neighbours, count neighbours.
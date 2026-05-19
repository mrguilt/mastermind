# Mastermind

This is the classic game Mastermind, written in Rust. I'm doing this as a programming exercise (much the same way, in college, I wrote it in Pascal). 

## Basic Play

The goal is to guess the sequence of colored pegs the computer picks. The pegs are:

* **R**ed
* **Y**ellow
* **G**een
* **B**lue. 

After each guess, the computer will award you between zero and four pegs. For each peg you guess the correct color *and* position, you will get a **B**lack peg. For each peg that is the correct color but incorrect position, you'll get a **W**hite peg. 

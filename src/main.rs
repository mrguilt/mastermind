// Mastermind
// A Rust implementation of the game Mastermind.
// Created by I. Charles Barilleaux
// Created on 2026-05-18

const COLORS:[char;4]=['r','y','g','b']; //colors of the pegs
 
fn main() {
    println!("Hello, world!");
    for element in COLORS {
        println!("{} is {}",element,color_to_number(element.to_ascii_lowercase()));
    }
    let test='C';
    println!("{} is {}",test, test.to_ascii_lowercase());
}

//Returns the array index matching to the peg color. 
fn color_to_number (x: char) -> u8 {
    let mut result=0;
    let mut counter=0;
    while((counter<COLORS.len())) {
        if x == COLORS[counter] {
            result=counter;
        }
        counter+=1;
    }
    let outcome=result as u8;
    outcome
}
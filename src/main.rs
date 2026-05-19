// Mastermind
// A Rust implementation of the game Mastermind.
// Created by I. Charles Barilleaux
// Created on 2026-05-18
use std::io;

use std::cmp::Ordering; //Listing 2-4
use rand::Rng; //Listing 2-3

const COLORS:[char;4]=['r','y','g','b']; //colors of the pegs
 
fn main() {
//    let mut rng = rand::thread_rng(); // Get a thread-local random number generator

    println!("Mastermind for Rust");
    println!("========== === ====\n");
    print!("Target:\t");
    generate_target();
    println!("\n");
    
    loop { 
//        let mut fullguess=String::new();
        let mut guess=String::new();

        println!("Enter your guess: ");
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let mut fullguess=guess.trim();
        if fullguess.trim() == "quit" {
            break;
        }

        let mut guess:Vec<&str>=fullguess.split(',').collect(); 
//        println!("length: {}",guess.len());

        if guess.len()!=4 {
            println!("INCORRECT GUESS FORMAT!!! Please enter 4 \"pegs\" (p,p,p,p).");
        } else {
            for element in guess {
                println!("{} is {}",element,color_to_number(element.chars().next().unwrap()));
            }
        }
    }

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

//rand::thread_rng().gen_range(1..=100);

fn generate_target() {
    for number in (0..4) {
        let peg = rand::thread_rng().gen_range(0,4);
        print!("{}",COLORS[peg]);
        if number<3 {
                print!(",");
        }
    }
    print!("\n");
}
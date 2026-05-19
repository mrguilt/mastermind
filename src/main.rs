// Mastermind
// A Rust implementation of the game Mastermind.
// Created by I. Charles Barilleaux
// Created on 2026-05-18
use std::io;

use std::cmp::Ordering; //Listing 2-4
use rand::Rng; //Listing 2-3

const COLORS:[char;4]=['r','y','g','b']; //colors of the pegs
 
fn main() {
    println!("Mastermind for Rust");
    println!("========== === ====\n");
// Set what the "target" peg position
    print!("Target:\t");
    let targetpegs=generate_target(); //Set the goal
    for element in targetpegs {
        print!("{}:",COLORS[element as usize]);
    }
    println!("\n");

    loop { 
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

// This will generate the target. 
fn generate_target() -> [u8;4] {
    let mut pegs:[u8;4]=[0,0,0,0];
    for number in (0..4) {
        pegs[number] = rand::thread_rng().gen_range(0,4);
    }
    pegs
}

//This is my test generate target. I'm keeping it until I get the main set.
fn test_generate_target() {
    for number in (0..4) {
        let peg = rand::thread_rng().gen_range(0,4);
        print!("{}",COLORS[peg]);
        if number<3 {
                print!(",");
        }
    }
    print!("\n");
}
// Mastermind
// A Rust implementation of the game Mastermind.
// Created by I. Charles Barilleaux
// Created on 2026-05-18
use std::io;
use std::cmp::Ordering; 
use rand::Rng; 

const COLORS:[char;4]=['r','y','g','b']; //colors of the pegs
const MAXGUESS:u8=10;
 
fn main() {
    println!("Mastermind for Rust");
    println!("========== === ====\n");
    print!("Target:\t");
    let targetpegs=generate_target(); //Set the goal
    printpegs(targetpegs); //REMOVE ME FOR FINAL I will want to see this to debug. 
    println!("\n");

    let mut guesscount=1;
    loop { 
        let mut guess=String::new();

        println!("Guess #{}. Enter your guess: ",guesscount);
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let mut fullguess=guess.trim();
        if guess.trim() == "quit" {
            break;
        }

        let mut guesspegs=[0;4];
        let mut mypegs:Vec<&str>=fullguess.split(',').collect(); 
        if mypegs.len()!=4 {
            println!("INCORRECT GUESS FORMAT!!! Please enter 4 \"pegs\" (p,p,p,p).");
        } else {
            let mut number=0;
            for element in mypegs {
                //Convert the guess to an array of numbers. 
                guesspegs[number]=color_to_number(element.chars().next().unwrap());
                number+=1;
            }
            guesscount+=1;
        }
        print!("Your guess: ");
        printpegs(guesspegs);
        println!("");

        if guesscount>MAXGUESS {
            println!("\nNo guesses remain!");
            print!("The actual result: ");
            printpegs(targetpegs);
            println!("");
            break;
        }

        println!("\t{} guesses remain.",(MAXGUESS-guesscount));

        println!("\n\n------------");
        println!("working my way through the comparison function...");
        checkguess(guesspegs,targetpegs);
        println!("\n\n------------");

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

//Takes an array of peg positions and prints them column separated (no newline)
fn printpegs(thesepegs:[u8;4]) {
    for number in (0..4) {
        print!("{}",COLORS[thesepegs[number] as usize]);
         if number<3 {
                print!(",");
        }
    }
}

fn checkguess(guesspegs:[u8;4],goalpegs:[u8;4]) {
    let mut checked:[bool;4]=[false;4];
    let mut results:[u8;2]=[0;2];

    println!("In the function");

    let mut guesscounter=0;
    'guess_count: loop {
        let mut targetcount=0;

        'target_count: loop {
            if(checked[targetcount]==false) {
                if(goalpegs[targetcount]==guesspegs[guesscounter]) {
                    checked[targetcount]=true;
                    if targetcount==guesscounter {
                        results[0]+=1;
                    } else {
                        results[1]+=1;
                   }
                   targetcount+=1;
                    break 'target_count;
                } else {
                }
            }
            targetcount+=1;
            if targetcount>=goalpegs.len() {
                break 'target_count;
            }
        }
        guesscounter+=1;
        if guesscounter>=guesspegs.len() {
            break 'guess_count;
        }
    }
    println!("Results:");
    println!("\tBlack Pegs: {}\tWhite Pegs: {}",results[0],results[1]);
}


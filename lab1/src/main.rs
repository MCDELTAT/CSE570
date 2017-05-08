// Author: Aaron Chamberlain
// Course: 570, Compilers
// Professor: Dr. Ernesto Gomez
// Quarter: Spring 2017

// Program Description: This program sets up a table-based deterministic finite-state automata that
// allows the user to test strings against the regular expression:
// (a|b)*abb
// Given the structure of the program, it can be adapted to any other regular expression.
// The user provides the input via arguments at run time. In this case, any string that ends in 'abb'
// and uses 0 or more 'a' or 'b' characters before that is accepted. Ex: 'abb', 'aabb', 'babb', 'ababb' etc.
// Any input string that includes characters not in the alphabet or that doesn't end in 'abb' will fail.

// Known inputs that behave oddly: Symbols that evaluate to path or other strings in Unix based systems work.
// I.E. '~' will evaluate to the string of your home directory and tested against.
// The following characters are not currently accepted: '`' '#' '&' '()' '\' '|' '<>'

use std::env;

// All of the configuration occurs here in these constant declarations as the table and all other
// structs are derived from these. Some descriptions will be provided below:
// EXPRESSION: This string is used only to let the user know what is being tested against.

// TABLE: This is the DFA written in the form of a 2D array. Each state has two possible transition
// cases, therefore, the next state is determined by what state you are currently in, and then the
// transition based off of the character input.

const EXPRESSION: &'static str = "(a|b)*abb";
const TABLE: [[usize; 2]; 4] = [[1,0], [1,2], [1,3], [1,0]];
const ALPHABET: [char; 2] = ['a', 'b']; // ALPHABET: The letters that are in the reg. ex. and can be matched against.
const START_STATE: usize = 0; // START_STATE: The state to start in
const HALT_STATE: usize = 3; // HALT_STATE: The accepting state
const STATE_COUNT: usize = 4; // STATE_COUNT: The number of possible states
const SYMBOL_COUNT: usize = 2; // SYMBOL_COUNT: The number of characters in the alphabet

enum RegexResult { Success } // the return type of a successful match at halting state

// A struct for the DFA for cleaner code. each DFA can have it's own table to match against, and can
// keep track of it's state and whether it reached an accepting state. Good for if you want to
// compare against more than one regular expression at once.
struct DFA {
    table: [[usize; SYMBOL_COUNT]; STATE_COUNT],
    state: usize,
    halt: usize
}

// A struct for the Language used. Used primarily for readability.
struct Language {
    alphabet: [char; SYMBOL_COUNT],
    size: usize
}

// By declaring in a global scope, it avoids having the var falling out of scope with each call of
// get_transition_value(). It also avoids passing unnesscessary references through the functions.
const LANGUAGE: Language = Language {alphabet: ALPHABET, size: SYMBOL_COUNT};

// function to determine the transition value, possible returns: 0 for 'a', 1 for 'b', 12 for anything else
fn get_transition_value(input_ch: char) -> usize {
    for i in 0..LANGUAGE.size {
        if input_ch == LANGUAGE.alphabet[i] {
            return i;
        }
    }
    // return not found. since I must return usize for i, this will ensure a unique value that
    // cannot be returned from above, because it is bigger than the largest iteration.
    return LANGUAGE.size + 10;
}

// Initializes the DFA and then carries out transitions based on the current character of the input.
// If the DFA ever reaches the halting state, then it passes Success, otherwise throws error.
fn regex_match(input_string: &str) -> Result<RegexResult, &'static str> {
    let mut automata = DFA {table: TABLE, state: START_STATE, halt: HALT_STATE};
    for i in 0..input_string.chars().count() {
        let ch = input_string.chars().nth(i).unwrap();
        let transition = get_transition_value(ch);
        if transition == LANGUAGE.size + 10 { return Err("Input Failed"); }
        automata.state = automata.table[automata.state][transition];
    }
    if automata.state == automata.halt {
        return Ok(RegexResult::Success);
    }
    return Err("Input Failed")
}

// The main function and parsing interface. Gathers the input arguments into a vector, where each
// argument is then tested against the regex for success or failure.
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide at least 1 string to test.");
        std::process::exit(0);
    }
    println!("Matching input to: {}", EXPRESSION);
    for i in 1..args.len() {
        println!("Input String is: {}", args[i]);
        let argument = &args[i];
        match regex_match(&argument) {
            Ok(RegexResult::Success) => println!("Input accepted!"),
            Err(e) => println!("{}", e)
        }
    }
}

/*  Rust Implementation of the cat command with some other features
 *
 *  How to use
 *
 *  ./rust-cat [FLAGS] [FILES]
 *
 *  rust-cat will concatenate input and output file.
 *  The result is printed to terminal.
 *  
 *
 *  Flags:
 *
 *      -v : print the result into terminal anyway
 *      -s : Search for words that match a given pattern. Only those word in the second file will be concatenated.
 *           Usage: ./rust-cat -s beach,car,boat, <- Finish the pattern with a comma   
 *      
 *      -c : Count a given set words in the files. 
 *           Usage: ./rust-cat -c house, horse,mouse, <- Finish the pattern with a comma 
 *           
 *      -o : Output in a file. 
 *           Usage: ./rust-cat -o output.txt ...
 *
 *
 * */

//TODO: Create custom Error type

use std::env; // for collecting arguments
use std::fs;  // File I/O


fn parse_arguments(args : &[String]) -> (&str, &str) {

    let file1 = &args[1];
    let file2 = &args[2];

    (file1, file2)
}

fn main() {
    
    let args : Vec<String> = env::args().collect();

    let (file1, file2) = parse_arguments(&args);

    let file1_content = fs::read_to_string(file1).expect("Something went wrong reading the first file");
    let file2_content = fs::read_to_string(file2).expect("Something went wrong reading the second file");

    println!("{}", file1_content);
    println!("{}", file2_content);

}

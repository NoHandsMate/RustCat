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
 *      -s : Search for words that match a given pattern. Only those word will be concatenated.
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


struct Settings {

    verbose : bool,
    search  : bool,
    count   : bool,
    output  : bool
}


impl Settings {
    
    fn new() -> Settings {
        
        Settings {
            verbose : false,
            search  : false,
            count   : false,
            output  : false
        }
    }
}


fn extract_pattern(source : &[String], settings : &mut Settings) -> Vec<String> {
    
    let mut word_pattern : Vec<String> = Vec::new();
    
    settings.search = true;
    

    for value in source.iter() {
        
        let mut last_word_index = 0;

        let sliced = &value[..];

        for (ind, character) in sliced.chars().enumerate() {
            
            if character == ',' {
                
                let new_slice = &sliced[last_word_index..ind];
                last_word_index = ind + 1;

                let string = String::from(new_slice);
                word_pattern.push(string);
            }

        }
        
    }
         
    word_pattern
}


fn main() {
    
    let args : Vec<String> = std::env::args().collect();
    
    let pattern : Vec<String> = Vec::new();
    let args = &args[1..];  // Remove the saved filename and extract only the args
    
    let mut settings = Settings::new();

    for (index, value) in args.iter().enumerate() { 
        
        let value = &value[..];

        match value {
            "-v" => {

               settings.verbose = true; 

            }

            "-s" => {
                
                pattern = extract_pattern(&args[index + 1..], &mut settings);
            }

            "-c" => {

               settings.count = true; 
            }


            "-o" => {


            }

            _ => ()
            
        }
        
    }

}

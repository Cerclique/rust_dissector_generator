use wirdigen::validator::Validator;
use wirdigen::generator::Generator;
use wirdigen::error::WirdigenError;

use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use std::env;

fn foo() -> Result<(), WirdigenError> {

    let mut args = env::args();
    if args.len() == 1 {
        println!("Usage: ./rust_dissector_generator <path_to_json>");
        return Ok(())
    }

    // Get user input
    let input_dissector_path = args.nth(1).expect("Invalid user input");
    
    // Open the JSON file
    let file = File::open(input_dissector_path)?;
    let rdr = BufReader::new(file);

    // Create serde JSON value from the file
    let value: Value = serde_json::from_reader(rdr)?;

    // Create Validator object
    let val = Validator::new()?;

    // Call to validation method
    if val.validate(&value) {
        // Create Generator object
        let mut gen = Generator::default();

        if cfg!(target_os = "linux") {
            let home_path: String  = env::var("HOME").expect("Failed to retrieve $HOME environmnt variable");
            let output_dir = format!("{}/.local/lib/wireshark/plugins", home_path);
            gen.set_output_directory(output_dir.as_str());
        }
        else {
            println!("Detected OS is not linux.");
            println!("Hence, output directory remains the default one: {}", gen.get_output_directory());
        }
        
        // Generate plugin from validated data
        let generated_file_path: String = gen.from_value(value)?;
        println!("{}", generated_file_path);
    }
    else {
        println!("Error occured while dissector validation");
    }
    
    Ok(())
}

fn main() -> Result<(), WirdigenError> {
    foo()
}

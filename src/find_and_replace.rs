use text_colorizer::*;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern : String,
    replace : String,
    input_file : String,
    output_file : String,
}

fn print_help(){
  eprintln!("{} - replace a string with a new string", "Finad and Replace".green());
  eprintln!("Example: {} {} {} {}", "<Target String>".blue(), "<Replacement String>".blue(), "input.txt".yellow(), "output.txt".yellow());
}

pub fn run(){
  let args: Vec<String> = std::env::args().skip(1).collect();
  if args.len() != 4{
      eprintln!("{} {} {} Got: { }", "Error: ".red().bold(), "Not enough arguments".yellow(), "Expected 4 arguments".yellow(), args.len().to_string().yellow());
      print_help();
      std::process::exit(1);
  }
}
use text_colorizer::*;
use std::env;
use std::fs;

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

fn read_and_write(args: &Arguments) {
  let data = match fs::read_to_string(&args.input_file) {
      Ok(v) => v,
      Err(e) => {
          eprintln!("{} {} {} {}", "Error:".red().bold(), "Input file".yellow(), args.input_file.yellow(), e);
          std::process::exit(1);
      }
  };

  match fs::write(&args.output_file, data) {
      Ok(v) => v,
      Err(e) => {
          eprintln!("{} {} {} {}", "Error:".red().bold(), "Output file".yellow(), args.output_file.yellow(), e);
          std::process::exit(1);
      }
  };
}

fn parse_args() -> Arguments {
  let args: Vec<String> = std::env::args().skip(1).collect();
  if args.len() != 4{

      eprintln!("{} {} {} Got: { }", "Error: ".red().bold(), "Not enough arguments".yellow(), "Expected 4 arguments".yellow(), args.len().to_string().yellow());
      print_help();
      std::process::exit(1);
  }

  Arguments{
      pattern: args[0].clone(),
      replace: args[1].clone(),
      input_file: args[2].clone(),
      output_file: args[3].clone(),
  }
}

pub fn run(){
  let args = parse_args();
  read_and_write(&args);
}

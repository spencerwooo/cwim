extern crate unicode_segmentation;

// Dealing with CLI arguments
use structopt::StructOpt;

// Graceful error handling
use exitfailure::ExitFailure;
use failure::ResultExt;

use regex::Regex;
// use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

// Cli argument container
#[derive(StructOpt)]
struct Cli {
  // The path to the file
  #[structopt(parse(from_os_str))]
  path: std::path::PathBuf,
}

fn replace_whitespace(input: &str, placeholder: &str, re: &Regex) -> String {
  re.replace_all(input, placeholder).into()
}

fn main() -> Result<(), ExitFailure> {
  println!("cwim - Count Words Inside a Markdown file.");

  let args = Cli::from_args();
  let arg_path = &args.path;

  // See here for error handling: https://rust-cli.github.io/book/tutorial/errors.html
  let content = std::fs::read_to_string(arg_path)
    .with_context(|_| format!("Could not read file `{}`", arg_path.to_string_lossy()))?;

  let mut word_count = 0;
  let mut line_count = 0;
  let mut line_with_blank_count = 0;

  // regex to remove unnecessary whitespace inside markdown file
  // see VS Code documentation: https://vscode-docs.readthedocs.io/en/stable/extensions/example-word-count/
  let whitespace_re = Regex::new(
    r"(?x)
  (< ([^>]+)<)
  -
  ^\s\s*
  -
  \s\s*$
  ",
  )
  .unwrap();
  // match multiple spaces and change to single space
  let multiple_spaces_re = Regex::new(r"\s+").unwrap();
  // match links and files in grammar "[](...)"
  let link_re = Regex::new(r"\]\((.*?)\)").unwrap();

  // process document
  for line in content.lines() {
    line_with_blank_count = line_with_blank_count + 1;
    let clean_line = String::from(line.trim());

    if !clean_line.is_empty() {
      // consider as a valid line
      line_count += 1;

      // remove whitespace
      let clean_line = replace_whitespace(&clean_line, "", &whitespace_re);
      let clean_line = multiple_spaces_re.replace_all(&clean_line, " ");
      let clean_line = link_re.replace_all(&clean_line, "]");

      // split words using unicode standards
      let words: Vec<&str> = clean_line.unicode_words().collect();
      word_count = word_count + words.len();

      // debug
      // for word in words {
      //   println!("{}", word);
      // }
    }
  }

  // reading time considering 200-250 wpm
  let reading_time = word_count / 250;

  println!("Total lines: {}", line_with_blank_count);
  println!("Total non-blank lines: {}", line_count);
  println!("Total words: {}", word_count);
  if reading_time < 1 {
    println!("Reading time: less than 1 min");
  } else {
    println!("Reading time: {} mins", reading_time);
  }
  Ok(())
}

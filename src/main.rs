extern crate unicode_segmentation;
extern crate walkdir;

// Dealing with CLI arguments
use structopt::StructOpt;

// Graceful error handling
use exitfailure::ExitFailure;
use failure::ResultExt;

// Find markdown files recursively
use walkdir::WalkDir;

use regex::Regex;
use std::fs;
use std::path::PathBuf;
// use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

// Cli argument container
#[derive(StructOpt)]
struct Cli {
  // The path to the file
  #[structopt(parse(from_os_str))]
  path: PathBuf,
}

fn replace_whitespace(input: &str, placeholder: &str, re: &Regex) -> String {
  re.replace_all(input, placeholder).into()
}

fn process_path(path: PathBuf) -> Vec<PathBuf> {
  let mut list_of_md_file_path: Vec<PathBuf> = vec![];

  // fetch path's metadata
  let md = fs::metadata(path.clone()).unwrap();

  if md.is_dir() {
    // directory, find all underlying markdown files
    for entry in WalkDir::new(path)
      .follow_links(true)
      .into_iter()
      .filter_map(|e| e.ok())
    {
      let file_name = entry.file_name().to_string_lossy();
      if file_name.ends_with(".md") {
        list_of_md_file_path.push(entry.path().to_path_buf());
      }
    }
    return list_of_md_file_path;
  } else if md.is_file() {
    // single file
    list_of_md_file_path.push(path);
    return list_of_md_file_path;
  } else {
    panic!("File path is invalid.");
  }
}

fn main() -> Result<(), ExitFailure> {
  println!("cwim - Count Words Inside a Markdown file.");

  let args = Cli::from_args();

  let list_of_md_files = process_path(args.path);
  for entry in list_of_md_files {
    println!("-------------------------------");
    println!("{}", entry.to_string_lossy());

    // See here for error handling: https://rust-cli.github.io/book/tutorial/errors.html
    let content = std::fs::read_to_string(entry.clone())
      .with_context(|_| format!("Could not read file `{}`", entry.to_string_lossy()))?;

    let mut word_count = 0;
    let mut line_count = 0;
    let mut blank_lines_count = 0;

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
      } else {
        blank_lines_count = blank_lines_count + 1;
      }
    }

    // reading time considering 200-250 wpm
    let reading_time = word_count / 250;

    println!("All lines: {} lines", line_count + blank_lines_count);
    println!("Blank lines: {} lines", blank_lines_count);
    println!("Total words: {} words", word_count);
    if reading_time < 1 {
      println!("Reading time: less than 1 min");
    } else {
      println!("Reading time: {} mins", reading_time);
    }
  }

  Ok(())
}

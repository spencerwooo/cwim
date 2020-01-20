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
use std::time::Instant;
// use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

// Cli argument container
#[derive(StructOpt)]
#[structopt(name = "cwim - Count Words Inside a Markdown file.")]
struct Cli {
  /// Verbose mode (-v, -vv, etc.)
  #[structopt(short, long, parse(from_occurrences))]
  verbose: u8,

  /// Target file or target directory
  #[structopt(name = "FILE_OR_PATH", parse(from_os_str))]
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
  let args = Cli::from_args();

  // get cwim version
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  // measure elapsed time
  let now = Instant::now();

  // find all markdown files
  let list_of_md_files = process_path(args.path);

  let mut total_word_count = 0;
  let mut total_line_count = 0;
  let mut total_blank_line_count = 0;
  let total_file_count = list_of_md_files.len();

  // count words inside markdown file
  for entry in list_of_md_files {
    if args.verbose > 0 {
      println!(
        "{}",
        std::iter::repeat("-")
          .take(entry.to_string_lossy().len())
          .collect::<String>()
      );
      println!("{}", entry.to_string_lossy());
    }

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
        if args.verbose > 1 {
          for word in words {
            print!("{} / ", word);
          }
          println!("");
        }
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

    total_word_count = total_word_count + word_count;
    total_line_count = total_line_count + line_count;
    total_blank_line_count = total_blank_line_count + blank_lines_count;
  }

  // print stats
  let elapsed_time = now.elapsed().as_secs_f64();
  let count_file_speed = total_file_count as f64 / elapsed_time;
  let count_word_speed = total_word_count as f64 / elapsed_time;
  let total_reading_time = total_word_count / 250;
  println!(
    "github.com/spencerwooo/cwim v{} / T = {:.3}s / ({:.1} files/s {:.1} words/s)",
    VERSION, elapsed_time, count_file_speed, count_word_speed
  );

  if total_reading_time < 1 {
    println!("Total reading time: less than 1 min");
  } else {
    println!("Total reading time: {} mins", total_reading_time);
  }

  Ok(())
}

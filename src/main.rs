#[macro_use]
extern crate prettytable;
extern crate unicode_segmentation;
extern crate walkdir;

// Dealing with CLI arguments
use structopt::StructOpt;

// Graceful error handling
use exitfailure::ExitFailure;
use failure::ResultExt;

// Find markdown files recursively
use walkdir::WalkDir;

// unicode word seperator (CJK compatible)
use unicode_segmentation::UnicodeSegmentation;

// print tabular outputs
use prettytable::format;
use prettytable::Table;

use std::fs;
use regex::Regex;
use std::time::Instant;
use std::path::PathBuf;
use std::collections::HashMap;


// Cli argument container
#[derive(StructOpt)]
#[structopt(name = "cwim - Count Words Inside a Markdown file. (CJK compatible)")]
struct Cli {
  /// Verbose mode (-v, -vv, etc.)
  #[structopt(short, long, parse(from_occurrences))]
  verbose: u8,

  /// Target file or target directory
  #[structopt(name = "FILE_OR_PATH", parse(from_os_str))]
  path: PathBuf,
}

// Stats for markdown file
#[derive(Hash, Eq, PartialEq, Debug)]
struct Stat {
  stat_all_lines: usize,
  stat_blank_lines: usize,
  stat_word_count: usize,
  stat_reading_time: String,
}

impl Stat {
  fn new(
    stat_all_lines: usize,
    stat_blank_lines: usize,
    stat_word_count: usize,
    stat_reading_time: String,
  ) -> Stat {
    Stat {
      stat_all_lines: stat_all_lines,
      stat_blank_lines: stat_blank_lines,
      stat_word_count: stat_word_count,
      stat_reading_time: stat_reading_time,
    }
  }
}

// replace whitespace according to regex pattern
fn replace_whitespace(input: &str, placeholder: &str, re: &Regex) -> String {
  re.replace_all(input, placeholder).into()
}

// strip path prefix
fn truncate_path(path: PathBuf, prefix: PathBuf) -> String {
  path
    .strip_prefix(prefix)
    .unwrap()
    .to_string_lossy()
    .to_string()
}

// find all markdown files recursively
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
  let base = args.path.clone();

  // get cwim version
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  // measure elapsed time
  let now = Instant::now();

  // find all markdown files
  let list_of_md_files = process_path(args.path);

  // all words in query
  let mut total_word_count = 0;
  // all lines in query
  let mut total_line_count = 0;
  // all blank lines in query
  let mut total_blank_line_count = 0;
  // file count in query
  let total_file_count = list_of_md_files.len();
  // push all stats in hashmap
  let mut stats = HashMap::new();

  // customize table output
  let stats_table_format = format::FormatBuilder::new()
    .column_separator('|')
    .separators(
      &[
        format::LinePosition::Top,
        format::LinePosition::Bottom,
        format::LinePosition::Title,
      ],
      format::LineSeparator::new('-', '+', '+', '+'),
    )
    .padding(1, 1)
    .build();

  let mut stats_table = Table::new();
  stats_table.set_format(stats_table_format);
  stats_table.set_titles(row![
    bFb -> "File name",
    Fb -> "all lines",
    Fb -> "blank lines",
    Fb -> "words",
    Fb -> "reading time"
  ]);

  // count words inside markdown file
  for entry in list_of_md_files.clone() {
    // debug
    if args.verbose > 0 {
      println!(
        "[DEBUG] {}",
        std::iter::repeat("-")
          .take(entry.to_string_lossy().len())
          .collect::<String>()
      );
      println!("[DEBUG] {}", entry.to_string_lossy());
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
          print!("[DEBUG] ");
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
    let reading_time_fmt;
    if reading_time < 1 {
      reading_time_fmt = "~1 min".to_string();
    } else {
      reading_time_fmt = format!("{}{}", reading_time, " mins");
    }

    // debug
    if args.verbose > 1 {
      println!(
        "[DEBUG] All lines: {} lines",
        line_count + blank_lines_count
      );
      println!("[DEBUG] Blank lines: {} lines", blank_lines_count);
      println!("[DEBUG] Total words: {} words", word_count);
      if reading_time < 1 {
        println!("[DEBUG] Reading time: less than 1 min");
      } else {
        println!("[DEBUG] Reading time: {} mins", reading_time);
      }
    }

    stats.insert(
      entry.clone(),
      Stat::new(
        line_count + blank_lines_count,
        blank_lines_count,
        word_count,
        reading_time_fmt,
      ),
    );

    total_word_count = total_word_count + word_count;
    total_line_count = total_line_count + line_count + blank_lines_count;
    total_blank_line_count = total_blank_line_count + blank_lines_count;

    let truncate_entry = truncate_path(entry.clone(), base.clone());
    stats_table.add_row(row![
      truncate_entry,
      stats[&entry].stat_all_lines,
      stats[&entry].stat_blank_lines,
      stats[&entry].stat_word_count,
      stats[&entry].stat_reading_time
    ]);
  }

  // print stats
  let elapsed_time = now.elapsed().as_secs_f64();
  let count_file_speed = total_file_count as f64 / elapsed_time;
  let count_word_speed = total_word_count as f64 / elapsed_time;
  let total_reading_time = total_word_count / 250;
  let total_reading_time_fmt = if total_reading_time < 1 {
    "~1 min".to_string()
  } else {
    format!("{}{}", total_reading_time, " mins")
  };

  println!(
    "cwim - Count Words Inside a Markdown file. (CJK compatible)\n\n  Found {} markdown file(s).\n",
    total_file_count
  );
  println!(
    "  github.com/spencerwooo/cwim  v{}  T={:.3}s  ({:.1} files/s {:.1} words/s)\n",
    VERSION, elapsed_time, count_file_speed, count_word_speed
  );

  stats_table.add_row(row![
    bFm -> "SUM",
    Fm -> total_line_count,
    Fm -> total_blank_line_count,
    Fm -> total_word_count,
    Fm -> total_reading_time_fmt
  ]);

  stats_table.printstd();

  Ok(())
}

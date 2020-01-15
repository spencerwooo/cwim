// Dealing with CLI arguments
use structopt::StructOpt;
// Graceful error handling
use exitfailure::ExitFailure;
use failure::ResultExt;

// Cli argument container
#[derive(StructOpt)]
struct Cli {
  // The pattern to look for
  pattern: String,
  // The path to the file
  #[structopt(parse(from_os_str))]
  path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
  println!("cwim - Count Words Inside a Markdown file.");

  let args = Cli::from_args();
  let arg_path = &args.path;
  let arg_pattern = &args.pattern;

  // See here for error handling: https://rust-cli.github.io/book/tutorial/errors.html
  let content = std::fs::read_to_string(arg_path)
    .with_context(|_| format!("Could not read file `{}`", arg_path.to_string_lossy()))?;

  for line in content.lines() {
    if line.contains(arg_pattern) {
      println!("{}", line);
    }
  }
  Ok(())
}

<div align="center">
  <img src="assets/icon.png" alt="icon" width="80px"/>

  <h1>cwim</h1>

  🎰 <em>cwim - Count Words Inside a Markdown file. (CJK friendly)</em>

  [![](https://github.com/spencerwooo/cwim/workflows/CI%20Release/badge.svg)](https://github.com/spencerwooo/cwim/actions)
  ![](https://img.shields.io/badge/CJK-friendly-1bb7ea?logo=markdown)
  ![](https://img.shields.io/badge/rust-2018-000000?logo=rust)
  [![GitHub](https://img.shields.io/github/license/spencerwooo/cwim)](LICENSE)
</div>

## Welcome

**c • wim** /c-wɪm/

```
cwim - Count Words Inside a Markdown file. (CJK friendly) 0.1.0

USAGE:
    cwim [FLAGS] <FILE_OR_PATH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, etc.)

ARGS:
    <FILE_OR_PATH>    Target file or target directory
```

`cwim` is a command line tool for counting words inside a markdown file / markdown files. Written in pure Rust, `cwim` is fast, minimal and is compatible with almost any Unicode text segmentation. `cwim` is built attempting to mimic the functionalities of [`cloc` - Count lines of code](https://github.com/AlDanial/cloc).

_**Note:** I wrote this to study Rust in the first place, meaning `cwim` has not been proven to be production-ready._

## Usage

We can run `cwim` against a single Markdown file:

```bash
cwim <MARKDOWN_FILE_NAME>.md # .mdown || .markdown
```

![](https://i.loli.net/2020/01/21/5FZEAgs6ymYvU3u.png)

We can also feed `cwim` a folder / directory path:

```bash
cwim <DIRECTORY_PATH>
```

![](https://i.loli.net/2020/01/21/IfrKHMux7eoAhL2.png)

`cwim` is fast even when counting many files:

```bash
cwim <LARGE_DIRECTORY_WITH_MARKDOWN_FILES>
```

![](https://i.loli.net/2020/01/21/zDlUrhcOHCba951.png)

Run `cwim -h` for more information:

```bash
cwim -h
```

![](https://i.loli.net/2020/01/21/7Y9WXebxtCDhnz4.png)

## Development

You'll need to install Rust on your local machine first, that includes `rustc`, `cargo` and other necessary toolkits. Then:

- Build project:

```bash
cargo build
```

- Run `cwim`:

```bash
cargo run
```

- Run `cwim` with command line arguments:

```bash
cargo run -- <ARGS>
```

## How does cwim work?

**cwim** first removes whitespace / blank lines / special characters / links / file urls and other word-irrelevant characters out of the Markdown file, then it uses the library [`unicode-segmentation`](https://github.com/unicode-rs/unicode-segmentation) to split text into Unicode words, then counts the words based on the list returned.

For more information:

- Run the following command to show individual file listings:

```bash
cwim <FILE_OR_PATH> -v
```

- Run the following command to see how the words are separated:

```bash
cwim <FILE_OR_PATH> -vv
```

---

🎰 **cwim** ©Spencer Woo. Released under the [MIT License](LICENSE).

Authored and maintained by Spencer Woo.

[@Portfolio](https://spencerwoo.com/) · [@Blog](https://blog.spencerwoo.com/) · [@GitHub](https://github.com/spencerwooo)

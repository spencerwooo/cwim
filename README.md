<div align="center">
  <img src="assets/icon.png" alt="icon" width="80px"/>

  <h1>cwim</h1>

  🎰 <em>cwim - Count Words Inside a Markdown file. (CJK friendly)</em>

  ![](https://img.shields.io/badge/🔎-cwim-1bb7ea?style=flat-square)
  ![](https://img.shields.io/badge/CJK-friendly-orange?style=flat-square&logo=markdown)
  ![](https://img.shields.io/badge/rust-2018-000000?style=flat-square&logo=rust)
</div>

## Welcome

**c • wim** /c-wɪm/

`cwim` is a command line tool for counting words inside a markdown file / markdown files. Written in pure Rust, `cwim` is fast, minimal and is compatible with almost any Unicode text segmentation.

## Usage

We can run `cwim` against a single Markdown file:

We can also feed `cwim` a folder / directory path:

`cwim` is fast even when counting many files:

Run `cwim -h` for more information:

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
cargo run -- <args>
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

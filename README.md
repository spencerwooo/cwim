<div align="center">
  <img src="assets/icon.png" alt="icon" width="80px"/>

  <h1>cwim</h1>

  🎰 <em>cwim - Count Words Inside a Markdown file. (CJK friendly)</em>

  ![](https://img.shields.io/badge/CJK-friendly-orange?logo=markdown)
  ![](https://img.shields.io/badge/rust-2018-000000?logo=rust)
  ![](https://github.com/spencerwooo/cwim/workflows/CI%20Release/badge.svg)
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

`cwim` is a command line tool for counting words inside a markdown file / markdown files. Written in pure Rust, `cwim` is fast, minimal and is compatible with almost any Unicode text segmentation.

## Usage

We can run `cwim` against a single Markdown file:

![](https://i.loli.net/2020/01/21/ur2tFDelKhYI6vO.png)

We can also feed `cwim` a folder / directory path:

![](https://i.loli.net/2020/01/21/gkAD12RmMcypsxK.png)

`cwim` is fast even when counting many files:

![](https://i.loli.net/2020/01/21/EyTve6gr2zQZK3M.png)

Run `cwim -h` for more information:

![](https://i.loli.net/2020/01/21/Lo8nRfyOjxkY6V7.png)

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

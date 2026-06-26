# mpc_preprocessor
Tool for preprocessing pixel-art images before upload to makeplayingcards.com

## Usage

1. Install [Rust](https://rust-lang.org/tools/install/). You might need to restart your terminal.
2. `cd` into this repo and install with `cargo install --path .`
3. Put all your images in a directory with nothing else in it.
4. Run `mpc_preprocessor <input_dir> <output_dir>`.
   `<output_dir>` will be created if it doesn't exist, and will be overwritten if it does exist.
5. Now go face the final boss: the makeplayingcards UI.


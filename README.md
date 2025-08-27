# TP
## Setup
- run `cp tp.sh.example tp.sh`
- replace `~/your/path/to/tp/target/release/tp` with your path to the release build
    - if you haven't build a release yet, you can run `cargo build --release`
- you may need to run `chmod +x tp.sh`
- add this line with the correct path in your `.zshrc` file
```
alias tp="source /your/path/to/tp/tp.sh"
```
## Using
- run `tp --help` to see commands and usage

# rust-grep
Recreating basic grep functionalities in Rust. Exercise part of The Rust Book.

## Building
To build this program you will need to have installed the Rust Programming Language. For a tutorial on this, check https://www.rust-lang.org/tools/install.
Running ```cargo build --release``` will build the binary in the ```target/release``` directory. Alternatively, you can build the binary and run it at the same time using ```cargo run --release```.


## Usage
```./rust_grep [QUERY] [FILE NAME]```

* **[QUERY]** - The query that will be searched for in the specified stream.
* **[FILE NAME]** - OPTIONAL - If specified, the program will search for que query in the given file. If not, the program will search the query in the terminal's standard output.

Examples:
* ```./rust_grep "nevermore" theraven.txt ```
* ```cat theraven.txt | ./rust_grep "nevermore"```
Both examples will produce the same output.

*Please note:* The characater separating "rust" from "grep" is an underscore (_), not a hyphen.
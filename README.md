# Ethan Rosati

## File tree

```
.
├── src/
│   ├── main.rs             # Handle args, parsing, validation, and printing
│   ├── scanner.rs          # Scanner struct and implementation
│   ├── token.rs            # Token enum w/ internal data
│   └── parser/
│       ├── mod.rs          # Module exports and shared indentation helpers
│       ├── procedure.rs    # Procedure header, global declarations, body
│       ├── decl.rs         # Integer/object declaration selection
│       ├── decl_integer.rs # Integer declarations
│       ├── decl_obj.rs     # Object declarations
│       ├── decl_seq.rs     # Declaration sequences
│       ├── stmt.rs         # Statement selection
│       ├── stmt_seq.rs     # Statement sequences
│       ├── assign.rs       # Expression, object, subscript, and alias assignments
│       ├── if.rs           # If statements and optional else branches
│       ├── loop.rs         # For loops
│       ├── print.rs        # Print statements
│       ├── read.rs         # Read statements
│       ├── cond.rs         # Conditions, logical operators, and brackets
│       ├── cmpr.rs         # Equality and less-than comparisons
│       ├── expr.rs         # Addition and subtraction
│       ├── term.rs         # Multiplication and division
│       ├── factor.rs       # IDs, constants, object access, and parentheses
│       └── validation.rs   # Variable types and scope stack definitions
├── Cargo.lock              # no touchy :)
├── Cargo.toml              # Package settings and dependencies (none for now)
├── README.md               # Project overview and usage notes
├── tester.sh               # Provided tester w/ Rust runner
```

## Special features/comments
- note: this project is written in Rust
- The included tester script shows the necessary additions to use a Rust runner rather than a Python or C++ one; see lines `5-10`
- That script assumes you already have Rust installed. On Unix-like OS's this can be done with a single terminal command. Otherwise, see [the Rust installation instructions](https://rust-lang.org/tools/install/)
    - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- The script also assumes some flavor of Linux OS, which seems reasonable given the g++ call for the C++ case
- The finished parse tree is pretty-printed by implementing the existing Rust Display trait, allowing for easy and idiomatic printing

## Design description and parse tree rep
The parser was designed to follow the suggested architecture as closely as possible given the paradigm differences between Python and Rust; each non-terminal is its own enum and each production is a value in that enum. The only exception is non-terminals with a single production, which are declared as structs for convenience. The non-terminals have a constructor (new() method) that performs the parsing. This structure isn't ideal from a performance perspective due to the large amount of references (Box<> objects in Rust) created during the parsing, but it was relatively straightforward to implement.

## Test description and known bugs
The parser was tested at first using the provided test cases. Once my code succeeded with those, I created additional test cases to exercise more of the grammar rules and the potential error states, revealing some minor issues that were corrected. I also made cases that I anticipated would break my code, and improved my error messages that way. No major architectural changes were required during development.

No known bugs.
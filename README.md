# Ethan Rosati

## File tree

```
.
├── src/
│   ├── main.rs             # Handle args, parsing, validation, and execution
│   ├── scanner.rs          # Scanner struct and implementation
│   ├── token.rs            # Token enum w/ internal data
│   └── parser/
│       ├── mod.rs          # Module exports and shared indentation helpers
│       ├── procedure.rs    # Procedure header, global declarations, body
│       ├── decl.rs         # Integer/object declaration selection
│       ├── decl_integer.rs # Integer declarations
│       ├── decl_obj.rs     # Object declarations
│       ├── decl_seq.rs     # Declaration sequences
│       ├── function.rs     # Function definitions and the function map
│       ├── parameters.rs   # Formal parameter and call argument lists
│       ├── stmt.rs         # Statement selection
│       ├── stmt_seq.rs     # Statement sequences
│       ├── assign.rs       # Expression, object, subscript, and alias assignments
│       ├── if.rs           # If statements and optional else branches
│       ├── loop.rs         # For loops
│       ├── call.rs         # Procedure call statements
│       ├── print.rs        # Print statements
│       ├── read.rs         # Read statements
│       ├── cond.rs         # Conditions, logical operators, and brackets
│       ├── cmpr.rs         # Equality and less-than comparisons
│       ├── expr.rs         # Addition and subtraction
│       ├── term.rs         # Multiplication and division
│       ├── factor.rs       # IDs, constants, object access, and parentheses
│       ├── utils.rs        # Error exit helpers that print to stdout
│       └── variables.rs    # Variable types, scope/call stacks, and GC object counting
├── Cargo.lock              # no touchy :)
├── Cargo.toml              # Package settings and dependencies (none for now)
├── README.md               # Project overview and usage notes
└── tester.sh               # Provided tester w/ Rust runner
```

## Special features/comments
- The program's existing structure using the built-in Rc type to wrap object variables and declaring custom types throughout paid dividends for this project. Rather than implementing extensive separate reference counting logic, the Rc already acts as a smart pointer that automatically drops (deallocates) an object when it becomes unreachable by strong references. Keeping a count of reachable objects was then as simple as incrementing and decrementing a global variable from HeapObj's constructor and destructor, respectively. Since this was implemented on a struct wrapped by ObjType, no code outside *variables.rs* and the `id = new object...` path of *assign.rs* was changed.
- note: this project is written in Rust
- The included tester script shows the necessary additions to use a Rust runner rather than a Python or C++ one; see lines `5-10`
- That script assumes you already have Rust installed. On Unix-like OS's this can be done with a single terminal command. Otherwise, see [the Rust installation instructions](https://rust-lang.org/tools/install/)
    - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- The script also assumes some flavor of Linux OS, which seems reasonable given the g++ call for the C++ case

## Test description and known bugs
Very little specialized testing was needed for this project. I ran a couple test programs with print statements to verify the Rc tracked references as I expected; this test also verified new and drop would run when I anticipated. From there, I implemented the global variable with the sum of reachable objects and validated against the provided test cases. I was confident in the implementation from that testing since it almost entirely reused working code from prior projects and I had already validated the additions in `variables.rs` through my Rc testing.

No known bugs.
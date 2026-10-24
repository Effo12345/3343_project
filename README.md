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
│       ├── utils.rs        # Error exit helpers that print to stdout
│       └── variables.rs    # Variable types and scope stack definitions
├── Cargo.lock              # no touchy :)
├── Cargo.toml              # Package settings and dependencies (none for now)
├── README.md               # Project overview and usage notes
└── tester.sh               # Provided tester w/ Rust runner
```

## Special features/comments
- note: this project is written in Rust
- The included tester script shows the necessary additions to use a Rust runner rather than a Python or C++ one; see lines `5-10`
- That script assumes you already have Rust installed. On Unix-like OS's this can be done with a single terminal command. Otherwise, see [the Rust installation instructions](https://rust-lang.org/tools/install/)
    - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- The script also assumes some flavor of Linux OS, which seems reasonable given the g++ call for the C++ case
- Moderate levels of witchcraft were employed to allow multiple objects to hold a mutable reference to the underlying data structure. This is something Rust tends to avoid.

## Design description and variable scope handling
The interpreter was designed to mimic the already-working semantic checks as much as possible. To that end, it recurses the tree in a similar manner and manages variables and scope using the same data structure, VarStack. This is a stack of HashMaps mapping strings (identifiers) to variable type enums (indirectly, through ScopedVar). With this, the execute() methods can freely push new scopes onto the stack and pop them as needed. Variables are accessed by searching from the top of the stack for the first map containing a matching identifier, enforcing scoping and shadowing rules.

The variable enums themselves contain their data representations, with an Integer holding an i32 and an Object a map from String to i32 and a separate String as its default value. These types were chosen to most closely resemble the behavior of Java/Python types.

## Test description and known bugs
As with the parser, I began by testing the interpreter with a small subset of the grammar, gradually building more functionality until it passed all the provided test cases. Once my code succeeded with those, I created additional test cases to exercise more potential failure modes, revealing some minor oversights that were corrected. Notably, the lack of wrapping for operations in term and expr caused a Rust panic where the specifications expect normal operation. I also made cases that I anticipated would break my code, and improved my error messages that way. No major architectural changes were required during development since the main design decisions were already validated in the parser stage.

No known bugs.
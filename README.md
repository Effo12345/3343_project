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
│       └── variables.rs    # Variable types, scope stacks, and call stack
├── Cargo.lock              # no touchy :)
├── Cargo.toml              # Package settings and dependencies (none for now)
├── README.md               # Project overview and usage notes
└── tester.sh               # Provided tester w/ Rust runner
```

## Special features/comments
- A global map of functions was used to provide easy runtime calling anywhere within the tree, rather than forcing the caller to store a reference to the procedure object elsewhere in the tree. This plays better with Rust's borrowing rules
- note: this project is written in Rust
- The included tester script shows the necessary additions to use a Rust runner rather than a Python or C++ one; see lines `5-10`
- That script assumes you already have Rust installed. On Unix-like OS's this can be done with a single terminal command. Otherwise, see [the Rust installation instructions](https://rust-lang.org/tools/install/)
    - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- The script also assumes some flavor of Linux OS, which seems reasonable given the g++ call for the C++ case

## Design description and call stack
Implementing the call stack into the existing code was straightforward; it simply involved a type change to pass variables as a stack of stacks of maps, rather than just a stack of maps. This way, the outer stack entries become frames and the inner stack entries are the same scopes with the same logic as before. When the interpreter needs to access a variable, it searches only the top frame on the stack (the frame for the procedure it's running in) and the bottom scope of the bottom frame (the global variables declared during decl_seq). Scope resolution within a given frame works as before, with search for the identifier starting at the top of the scopes stack.

The other major design change was implementing procedure calls. This was handled by placing all the procedures into a map during parsing, then passing this map around during execution; procedure calls evaluate the arguments in their frame, get the Function struct from the map, then call its execute method. The Function struct is then responsible for pushing a new frame with the parameters.

## Test description and known bugs
Since this project had pieces of almost all the other projects (parser, validation, execution), I decided to implement and test each phase (i.e. unit testing) before moving on to the next. I began with the parser and validated it using the PrettyPrint methods. Then, the semantic checks, tested against the provided test cases. Once they permitted all the Correct cases and caught the expected Errors, I moved on to the interpreter, using the semantic checks as a model, as I did for project 3. After it succeeded for all the provided test cases, I wrote a few more to exercise some of the execution paths I was less confident in, like accessing global variables inside procedure calls.

No known bugs.
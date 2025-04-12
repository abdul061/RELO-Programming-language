[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**# RELO**

A custom-built programming language interpreter and compiler written in Rust, designed to resemble simple English-like syntax such as:

- `VARIABLE x IS 10;`
- `x ASSIGN x + 1;`
- `DISPLAY "Hello";`
- `WHEN condition { ... } OTHERWISE { ... } DONE;`
- `CREATE FUNCTION myfunc { ... } END_FN;`

**## 🚀 Features**

- ✅ Custom syntax with human-readable keywords (like `IS`, `ASSIGN`, `DISPLAY`, `START`, `END`)
- ✅ Support for:
  - Variable declarations and assignment
  - Conditionals (`WHEN`, `OTHERWISE`)
  - Loops (`REPEAT`, `LOOP`, `QUIT`)
  - Functions (`CREATE FUNCTION`, `CALL`, `RETURN`)
- ✅ Two execution modes:
  - Tree-walk interpreter
  - Bytecode compiler + virtual machine
- ✅ Rust-powered lexer, parser, and runtime
- ✅ Benchmark-ready with `cargo run --release`

---

## 📂 Project Structure

```text
.
├── lexer/         # Tokenizer for custom keywords and literals
├── parser/        # Builds AST from tokens
├── interpreter/   # Tree-walk interpreter
├── compiler/      # Compiler to bytecode
├── vm/            # Stack-based virtual machine
├── main.rs        # Program entry point
└── README.md

**🧪 Example Program**

START
    VARIABLE i IS 1;
    LOOP i Start 1 End 5 {
        DISPLAY i;
        i ASSIGN i + 1;
    } DONE;
END
Edit
START
    VARIABLE i IS 1;
    LOOP i Start 1 End 5 {
        DISPLAY i;
        i ASSIGN i + 1;
    } DONE;
END
**🏃 Running the Project
🔧 Debug (for development)**

cargo run
**⚡ Release (optimized for speed)**
bash

cargo run --release
🕒 Measure performance
Execution time is printed automatically after the program finishes.

**✍️ Author**
Abdul Mubarak H
📧 abdulmubarak682@gmail.com

****📜 License****
MIT License — Free to use, modify, and distribute.



---

Would you like me to:
- Add badges (build status, language, license)?
- Create sample test files?
- Include contribution or usage guidelines?

Let me know and I can customize it even further!








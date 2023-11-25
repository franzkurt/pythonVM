# pythonrepl
Just another python interpreter written in rust

### Goal

We don't imagine follow all PEPs or structures adapted from Cpython 3.12, we want to build some new project language that allows the core be most simple and focused on performance while accept lot of external modules like PyPI.

### Running Code

to run just install rustc and call `cargo run` into `/src` folder
Para rodar a aplicação instale o `rustc` e chame na linha de comando `cargo run` no diretorio `/src`

### structure

To build a VM we need to abstract the layer from Python language into Rust language, but indeed we will just use the operations provided by the rust to build or python execution pipeline. 

The steps are described bellow:

- [ ] Parse the syntax from python into Tokens (Python Parser)
- [ ] Build an AST (Abstract Syntax Tree) similar to ast python built in module
- [ ] Execute the generated AST into evaluate machine provided by rust layer (Evaluate Operations)
- [ ] Display the results os expressions or statements

### Development

Passing a str like `source_code` to the implementation return the following output.

```rust
fn main() {
    // interpreter::main();

    let source_code = "
import os, sys
import time
def func(a=45,b=\"42\"):
    print(a)
a = func";

    let tokens = Tokenizer(source_code);
    dbg!(&tokens);

    let mut parser = Parser::new(tokens);
    match parser.parse_tokens() {
        Ok(ast) => {dbg!(ast);}
        _ => {}
    }
}
```

Console call with `cargo run`

Tokens after parsing become this:

```
[src/main.rs:21] &tokens = [
    LineBreak,
    Keyword(
        Import,
    ),
    Space,
    Identifier(
        "os",
    ),
    Symbol(
        Comma,
    ),
    Space,
    Identifier(
        "sys",
    ),
    LineBreak,
    Keyword(
        Import,
    ),
    Space,
    Identifier(
        "time",
    ),
    LineBreak,
    Keyword(
        Def,
    ),
    Space,
    Identifier(
        "func",
    ),
    Symbol(
        LeftParen,
    ),
    Identifier(
        "a",
    ),
    Symbol(
        Assign,
    ),
    Text(
        "45",
    ),
    Symbol(
        Comma,
    ),
    Identifier(
        "b",
    ),
    Symbol(
        Assign,
    ),
    FString(
        "42",
    ),
    Symbol(
        RightParen,
    ),
    Symbol(
        Colon,
    ),
    LineBreak,
    IncrementIdent,
    Identifier(
        "print",
    ),
    Symbol(
        LeftParen,
    ),
    Identifier(
        "a",
    ),
    Symbol(
        RightParen,
    ),
    LineBreak,
    Identifier(
        "a",
    ),
    Space,
    Symbol(
        Assign,
    ),
    Space,
    Identifier(
        "func",
    ),
]
```

And AST becomes so an AST:

```
[src/main.rs:25] ast = [
    Import(
        "os",
    ),
    Import(
        "sys",
    ),
    Import(
        "time",
    ),
    Function(
        Function {
            name: "func",
            params: [
                (
                    "a",
                    Some(
                        Integer(
                            45,
                        ),
                    ),
                ),
                (
                    "b",
                    Some(
                        String(
                            "42",
                        ),
                    ),
                ),
            ],
            body: [],
        },
    ),
]
```

I am currently implementing the call for functions that shoul evaluate operations.


### Contribute

Just help to document, solve bugs or whetever, PR and I will accept if was helpfull. Thx :)


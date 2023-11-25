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

### Contribute

Just help to document, solve bugs or whetever, PR and I will accept if was helpfull. Thx :)


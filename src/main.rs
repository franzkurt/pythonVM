mod interpreter;
mod tokenizer;
mod parser;

use crate::tokenizer::Tokenizer;
use crate::parser::Parser;

#[allow(warnings)]

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

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main2() {
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            4+7,
            4-7,
            4*7,
            4*7,
        ];

        for val in vals {
            tx1.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
        }
    });

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         // thread::sleep(Duration::from_secs(1));
    //     }
    // });

    for received in rx {
        println!("Got: {}", received);
    }

}

extern crate linefeed;
extern crate parse_read;
extern crate lval;

use std::io;
use linefeed::*;
use parse_read::read::read;
use parse_read::parse::parse_risp;

use lval::lval_eval::lval_eval;
use lval::lval_def::Lenv;
use lval::lval_builtin::Lbuiltin;

fn main() -> io::Result<()>{
    let mut lenv = Lenv::new();
    Lbuiltin::add_builtins(&mut lenv);

    let interface = Interface::new("risp-repl")?;

    interface.set_prompt("risp> ")?;

    while let ReadResult::Input(line) = interface.read_line()? {
        println!("read input: {}", lval_eval(&lenv, &mut read(parse_risp(line.as_bytes()))));

        if !line.trim().is_empty() {
            interface.add_history_unique(line);
        }
    }

    Ok(())
}

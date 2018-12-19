use crate::lval::lval_def::*;
use crate::lval::lval_builtin::*;
use crate::eval::eval_rispreter;

use std::io;
use linefeed::*;

pub struct RispRepl {
    prelude: Lenv
}

impl RispRepl {
    pub fn new() -> Self {
        RispRepl {
            prelude: Lenv::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()>{

        Lbuiltin::add_builtins(&mut self.prelude);

        let interface = Interface::new("risp-repl")?;

        interface.set_prompt("risp> ")?;

        while let ReadResult::Input(line) = interface.read_line()? {
            println!("{}", eval_rispreter(&mut self.prelude, line.to_string()));

            if !line.trim().is_empty() {
                interface.add_history_unique(line);
            }
        }

        Ok(())
    }
}

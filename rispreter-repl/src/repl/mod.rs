use crate::lval::lval_env::Lenv;
use crate::lval::lval_builtin::*;
use crate::eval::eval_rispreter;

use std::io;
use linefeed::*;

pub struct RispRepl {
    env: Lenv
}

impl RispRepl {
    pub fn new() -> Self {
        RispRepl {
            env: Lenv::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()>{

        Lbuiltin::add_builtins(&mut self.env);

        let interface = Interface::new("risp-repl")?;

        interface.set_prompt("risp> ")?;

        while let ReadResult::Input(line) = interface.read_line()? {
            println!("{}", eval_rispreter(&mut self.env, line.to_string()));
            for child in self.env.children() {
                child.detach();
            }
            if !line.trim().is_empty() {
                interface.add_history_unique(line);
            }
        }

        Ok(())
    }
}

use crate::eval::eval_rispreter;
use crate::lval::lval_builtin::*;
use crate::lval::lval_env::Lenv;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;

use linefeed::*;
use std::io;

use clap::App;

#[derive(Default)]
pub struct RispRepl {
    env: Rc<Lenv>,
}

impl RispRepl {
    pub fn new() -> Self {
        RispRepl { env: Lenv::new() }
    }

    pub fn run(&self) -> io::Result<()> {
        Lbuiltin::add_builtins(&self.env);
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();
        let target_file = matches.value_of("INPUT_FILE");
        match target_file {
            Some(filename) => {
                let program = RispRepl::read_rispreter(filename);
                println!("{}", program);
                for lines in program.lines() {
                    println!("{}", eval_rispreter(&self.env, &lines));
                    // for child in self.env.children() {
                    //     child.detach();
                    // }
                }
            }
            None => {
                println!("Error in read file {:?}", target_file);
            }
        }

        let interface = Interface::new("risp-repl")?;

        interface.set_prompt("risp> ")?;

        while let ReadResult::Input(line) = interface.read_line()? {
            println!("{}", eval_rispreter(&self.env, &line));
            // for child in self.env.children() {
            //     child.detach();
            // }
            if !line.trim().is_empty() {
                interface.add_history_unique(line);
            }
        }

        Ok(())
    }

    fn read_rispreter(tmp: &str) -> String {
        let filename = Path::new(tmp);
        match File::open(Path::new(&filename)) {
            Ok(mut fh) => {
                let mut contents = String::new();
                match fh.read_to_string(&mut contents) {
                    Ok(_) => contents,
                    Err(err) => {
                        println!("There was an error reading file: {:?}", err);
                        std::process::exit(1);
                    }
                }
            }
            Err(err) => {
                println!("File not found: {:?}", err);
                std::process::exit(1);
            }
        }
    }
}

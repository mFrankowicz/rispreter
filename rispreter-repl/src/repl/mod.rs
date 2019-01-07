use crate::eval::eval_rispreter;
use crate::lval::lval_env::Lenv;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

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
        let prelude = "(fun {flip f a b} {f b a})
(fun {ghost & xs} {eval xs})
(fun {comp f g x} {f (g x)})
(fun {len l} { if (== l nil) {0} {+ 1 (len (tail l))} })
(fun {foldl f z l} { if (== l nil) {z} {foldl f (f z (fst l)) (tail l)} })
(fun {sum l} {foldl + 0 l})
(fun {product l} {foldl * 1 l})
(def {otherwise} true)
(fun {case x & cs} { if (== cs nil) {error \"No Case Found\"} {if (== x (fst (fst cs))) {snd (fst cs)} { unpack case (join (list x) (tail cs))}} })
(fun {fib n} { select { (== n 0) 0 } { (== n 1) 1 } { otherwise (+ (fib (- n 1)) (fib (- n 2))) } })
".to_string();

        //Lbuiltin::add_builtins(&self.env);
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

        println!("loading prelude library: ");
        for lines in prelude.lines() {
            println!("{}", eval_rispreter(&self.env, &lines));
        }
        println!("enjoy!");
        println!("_______________________________________________");
        println!("ctrl-d do quit\nctrl-l to clear buffer");
        println!("_______________________________________________");

        let interface = Interface::new("risp-repl")?;

        interface.set_prompt("> ")?;

        interface.define_function("enter-function", Arc::new(EnterFunction));
        interface.bind_sequence("\r", Command::from_str("enter-function"));
        interface.bind_sequence("\n", Command::from_str("enter-function"));

        interface.define_function("tab-function", Arc::new(TabFunction));
        interface.bind_sequence("\t", Command::from_str("tab-function"));

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

struct EnterFunction;

impl<Term: Terminal> Function<Term> for EnterFunction {
    fn execute(&self, prompter: &mut Prompter<Term>, count: i32, _ch: char) -> io::Result<()> {
        if prompter.buffer().ends_with('.') {
            prompter.accept_input()
        } else if count > 0 {
            prompter.insert(count as usize, '\n')?;
            prompter.insert(2, ' ')
        } else {
            Ok(())
        }
    }
}

struct TabFunction;
impl<Term: Terminal> Function<Term> for TabFunction {
    fn execute(&self, prompter: &mut Prompter<Term>, _count: i32, ch: char) -> io::Result<()> {
        if ch == '\t' {
            prompter.insert(4, ' ')
        } else {
            Ok(())
        }
    }
}

use crate::eval::eval_rispreter;
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
        let prelude = "(def {fun} (\\ {args body} {def (head args) (\\ (tail args) body)}))
(fun {unpack f xs} {eval (join (list f) xs)})
(fun {pack f & xs} {f xs})
(def {uncurry} pack)
(def {curry} unpack)
(def {nil} {})
(fun {do & l} { if (== l nil) {nil} {last l} })
(fun {let b} { ((\\ {_} b) ()) })
(fun {not x}   {- 1 x})
(fun {or x y}  {+ x y})
(fun {and x y} {* x y})
(fun {flip f a b} {f b a})
(fun {ghost & xs} {eval xs})
(fun {comp f g x} {f (g x)})
(fun {fst l} { eval (head l) })
(fun {snd l} { eval (head (tail l)) })
(fun {trd l} { eval (head (tail (tail l))) })
(fun {len l} { if (== l nil) {0} {+ 1 (len (tail l))} })
(fun {nth n l} { if (== n 0) {fst l} {nth (- n 1) (tail l)} })
(fun {last l} {nth (- (len l) 1) l})
(fun {take n l} { if (== n 0) {nil} {join (head l) (take (- n 1) (tail l))} })
(fun {drop n l} { if (== n 0) {l} {drop (- n 1) (tail l)} })
(fun {split n l} {list (take n l) (drop n l)})
(fun {elem x l} { if (== l nil) {false} {if (== x (fst l)) {true} {elem x (tail l)}} })
(fun {map f l} { if (== l nil) {nil} {join (list (f (fst l))) (map f (tail l))} })
(fun {filter f l} { if (== l nil) {nil} {join (if (f (fst l)) {head l} {nil}) (filter f (tail l))} })
(fun {foldl f z l} { if (== l nil) {z} {foldl f (f z (fst l)) (tail l)} })
(fun {sum l} {foldl + 0 l})
(fun {product l} {foldl * 1 l})
(fun {select & cs} { if (== cs nil) {error \"No Selection Found\"} {if (fst (fst cs)) {snd (fst cs)} {unpack select (tail cs)}} })
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

        interface.set_prompt(">>>> ")?;

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

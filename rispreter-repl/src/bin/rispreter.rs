#![feature(test)]

extern crate test;

use rispreter_repl::repl::RispRepl;
use std::io;

fn main() -> io::Result<()> {
    let risp_repl = RispRepl::new();
    //Ok(risp_repl.run_instruction("(fib 10)"))
    risp_repl.run()
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_fib(b: &mut Bencher) {
        let risp = RispRepl::new();
        b.iter( || risp.run_instruction("(fib 10)"))
    }
}

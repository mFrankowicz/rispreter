use std::io;
use rispreter_repl::repl::RispRepl;

fn main() -> io::Result<()>{
    let mut risp_repl = RispRepl::new();
    risp_repl.run()
}

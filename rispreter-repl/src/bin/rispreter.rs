use rispreter_repl::repl::RispRepl;
use std::io;

fn main() -> io::Result<()> {
    let risp_repl = RispRepl::new();
    risp_repl.run()
}

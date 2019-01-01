use crate::structure::Prelude;
use crate::structure::Risp;
use nom::types::CompleteStr;
use nom::*;

named!(lambda<CompleteStr, Risp>,
    do_parse!(tag!("\\") >> (Risp::LPrelude(Prelude::Lambda)))
);

named!(def<CompleteStr, Risp>,
    do_parse!(tag!("def") >> (Risp::LPrelude(Prelude::Def)))
);

named!(put<CompleteStr, Risp>,
    do_parse!(alt!(tag!("=") | tag!("put")) >> (Risp::LPrelude(Prelude::Put)))
);

named!(list<CompleteStr, Risp>,
    do_parse!(tag!("list") >> (Risp::LPrelude(Prelude::List)))
);

named!(head<CompleteStr, Risp>,
    do_parse!(tag!("head") >> (Risp::LPrelude(Prelude::Head)))
);

named!(tail<CompleteStr, Risp>,
    do_parse!(tag!("tail") >> (Risp::LPrelude(Prelude::Tail)))
);

named!(eval<CompleteStr, Risp>,
    do_parse!(tag!("eval") >> (Risp::LPrelude(Prelude::Eval)))
);

named!(join<CompleteStr, Risp>,
    do_parse!(tag!("join") >> (Risp::LPrelude(Prelude::Join)))
);

named!(cons<CompleteStr, Risp>,
    do_parse!(tag!("cons") >> (Risp::LPrelude(Prelude::Cons)))
);

named!(add<CompleteStr, Risp>,
    do_parse!(tag!("+") >> (Risp::LPrelude(Prelude::Add)))
);

named!(sub<CompleteStr, Risp>,
    do_parse!(tag!("-") >> (Risp::LPrelude(Prelude::Sub)))
);

named!(mul<CompleteStr, Risp>,
    do_parse!(tag!("*") >> (Risp::LPrelude(Prelude::Mul)))
);

named!(div<CompleteStr, Risp>,
    do_parse!(tag!("/") >> (Risp::LPrelude(Prelude::Div)))
);

named!(modulo<CompleteStr, Risp>,
    do_parse!(tag!("%") >> (Risp::LPrelude(Prelude::Mod)))
);

named!(rif<CompleteStr, Risp>,
    do_parse!(tag!("if") >> (Risp::LPrelude(Prelude::If)))
);

named!(eq<CompleteStr, Risp>,
    do_parse!(tag!("==") >> (Risp::LPrelude(Prelude::Eq)))
);

named!(neq<CompleteStr, Risp>,
    do_parse!(tag!("!=") >> (Risp::LPrelude(Prelude::Neq)))
);

named!(gt<CompleteStr, Risp>,
    do_parse!(tag!(">") >> (Risp::LPrelude(Prelude::Gt)))
);

named!(lt<CompleteStr, Risp>,
    do_parse!(tag!("<") >> (Risp::LPrelude(Prelude::Lt)))
);

named!(gte<CompleteStr, Risp>,
    do_parse!(tag!(">=") >> (Risp::LPrelude(Prelude::Gte)))
);

named!(lte<CompleteStr, Risp>,
    do_parse!(tag!("<=") >> (Risp::LPrelude(Prelude::Lte)))
);

named!(get<CompleteStr, Risp>,
    do_parse!(tag!("get") >> (Risp::LPrelude(Prelude::Get)))
);

named!(
    pub risp_prelude<CompleteStr, Risp>,
    alt!(
        lambda |
        def |
        list |
        head |
        tail |
        eval |
        join |
        cons |
        add |
        sub |
        mul |
        div |
        modulo |
        rif |
        eq |
        neq |
        gt |
        lt |
        gte |
        lte |
        get |
        put
    )
);

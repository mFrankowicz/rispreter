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

named!(fun<CompleteStr, Risp>,
    do_parse!(tag!("fun") >> (Risp::LPrelude(Prelude::Fun)))
);

named!(curry<CompleteStr, Risp>,
    do_parse!(alt!(tag!("curry") | tag!("unpack")) >> (Risp::LPrelude(Prelude::Curry)))
);

named!(uncurry<CompleteStr, Risp>,
    do_parse!(alt!(tag!("uncurry") | tag!("pack")) >> (Risp::LPrelude(Prelude::Uncurry)))
);

named!(nil<CompleteStr, Risp>,
    do_parse!(tag!("nil") >> (Risp::LPrelude(Prelude::Nil)))
);

named!(fst<CompleteStr, Risp>,
    do_parse!(tag!("fst") >> (Risp::LPrelude(Prelude::Fst)))
);

named!(snd<CompleteStr, Risp>,
    do_parse!(tag!("snd") >> (Risp::LPrelude(Prelude::Snd)))
);

named!(trd<CompleteStr, Risp>,
    do_parse!(tag!("trd") >> (Risp::LPrelude(Prelude::Trd)))
);

named!(nth<CompleteStr, Risp>,
    do_parse!(tag!("nth") >> (Risp::LPrelude(Prelude::Nth)))
);

named!(last<CompleteStr, Risp>,
    do_parse!(tag!("last") >> (Risp::LPrelude(Prelude::Last)))
);

named!(ldo<CompleteStr, Risp>,
    do_parse!(tag!("do") >> (Risp::LPrelude(Prelude::Do)))
);

named!(llet<CompleteStr, Risp>,
    do_parse!(tag!("let") >> (Risp::LPrelude(Prelude::Let)))
);

named!(not<CompleteStr, Risp>,
    do_parse!(tag!("not") >> (Risp::LPrelude(Prelude::Not)))
);

named!(and<CompleteStr, Risp>,
    do_parse!(tag!("and") >> (Risp::LPrelude(Prelude::And)))
);

named!(or<CompleteStr, Risp>,
    do_parse!(tag!("or") >> (Risp::LPrelude(Prelude::Or)))
);

named!(xor<CompleteStr, Risp>,
    do_parse!(tag!("xor") >> (Risp::LPrelude(Prelude::Xor)))
);

named!(select<CompleteStr, Risp>,
    do_parse!(tag!("select") >> (Risp::LPrelude(Prelude::Select)))
);

named!(take<CompleteStr, Risp>,
    do_parse!(tag!("take") >> (Risp::LPrelude(Prelude::Take)))
);

named!(drop<CompleteStr, Risp>,
    do_parse!(tag!("drop") >> (Risp::LPrelude(Prelude::Drop)))
);

named!(split<CompleteStr, Risp>,
    do_parse!(tag!("split") >> (Risp::LPrelude(Prelude::Split)))
);

named!(elemen<CompleteStr, Risp>,
    do_parse!(tag!("elemen") >> (Risp::LPrelude(Prelude::Elemen)))
);

named!(map<CompleteStr, Risp>,
    do_parse!(tag!("map") >> (Risp::LPrelude(Prelude::Map)))
);

named!(filter<CompleteStr, Risp>,
    do_parse!(tag!("filter") >> (Risp::LPrelude(Prelude::Filter)))
);

named!(list_op<CompleteStr, Risp>,
    alt!(
        fst |
        snd |
        trd |
        nth |
        last |
        list |
        head |
        tail |
        eval |
        join |
        cons |
        take |
        drop |
        split |
        elemen |
        map |
        filter
    )
);

named!(log_op<CompleteStr, Risp>,
    alt!(
        not |
        and |
        or |
        xor
    )
);

named!(
    pub risp_prelude<CompleteStr, Risp>,
    alt!(
        list_op |
        log_op |
        lambda |
        ldo |
        llet |
        select |
        fun |
        curry |
        uncurry |
        nil |
        def |
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

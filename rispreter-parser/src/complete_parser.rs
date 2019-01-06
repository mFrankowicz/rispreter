use crate::prelude::risp_prelude;
use crate::structure::NumType;
use crate::structure::Risp;
use crate::structure::TypedVec;

use nom::digit;
use nom::types::CompleteStr;
use nom::*;

named!(
    integer<CompleteStr, f64>,
    do_parse!(
        sign: opt!(tag!("-")) >>
        num: digit >>
        (
            {
                let mut tmp = String::from("");
                if sign.is_some() {
                    tmp.push_str("-");
                }
                tmp.push_str(&num);
                tmp.parse::<f64>().unwrap()
            }
        )
    )
);

named!(float<CompleteStr, f64>,
    do_parse!(
        sign: opt!(tag!("-")) >>
        left_nums: digit >>
        tag!(".") >>
        right_nums: digit >>
        (
            {
                let mut tmp = String::from("");
                if sign.is_some() {
                    tmp.push_str("-");
                }
                tmp.push_str(&left_nums.to_string());
                tmp.push_str(".");
                tmp.push_str(&right_nums.to_string());
                tmp.parse::<f64>().unwrap()
            }
        )
    )
);

named!(number<CompleteStr, f64>,
    alt!(integer | float)
);

// risp_comment
named!(
    risp_comment<CompleteStr, Risp>,
    ws!(
        do_parse!(
            tag!(";") >>
            take_until!("\n") >>
            (
                Risp::LComment
            )
        )
    )
);

// risp_symbol
named!(
    risp_symbol<CompleteStr, Risp>,
    do_parse!(
        content: is_a!("qwertyuiopasdfghjklçzxcvbnmQWERTYUIOPASDFGHJKLÇZXCVBNM1234567890_§?£¢¬~+-*/%&=!?><\\") >>
        (
            Risp::LSymbol(content.to_string())
        )
    )
);

// risp_integer
named!(
    risp_integer<CompleteStr, Risp>,
    do_parse!(
        sign: opt!(tag!("-")) >>
        num: digit >>
        (
            {
                let mut tmp = String::from("");
                if sign.is_some() {
                    tmp.push_str("-");
                }
                tmp.push_str(&num);
                Risp::LNumber(NumType::Int(tmp.parse::<i64>().unwrap()))
            }
        )
    )
);

// risp_float
named!(
    risp_float<CompleteStr, Risp>,
    do_parse!(
        sign: opt!(tag!("-")) >>
        left_nums: digit >>
        tag!(".") >>
        right_nums: digit >>
        (
            {
                let mut tmp = String::from("");
                if sign.is_some() {
                    tmp.push_str("-");
                }
                tmp.push_str(&left_nums.to_string());
                tmp.push_str(".");
                tmp.push_str(&right_nums.to_string());
                Risp::LNumber(NumType::Float(tmp.parse::<f64>().unwrap()))
            }
        )
    )
);

// risp_string
named!(
    risp_string<CompleteStr, Risp>,
    do_parse!(
        tag!("\"") >>
        content: take_until!("\"") >>
        tag!("\"") >>
        (
            Risp::LString(content.to_string())
        )
    )
);

// risp_char
named!(
    risp_char<CompleteStr, Risp>,
    do_parse!(
        tag!("\'") >>
        content: anychar >>
        tag!("\'") >>
        (
            Risp::LChar(content)
        )
    )
);

// risp_true
named!(
    risp_true<CompleteStr, Risp>,
    do_parse!(
        tag!("true") >>
        (
            Risp::LBool(true)
        )
    )
);

// risp_false
named!(
    risp_false<CompleteStr, Risp>,
    do_parse!(
        tag!("false") >>
        (
            Risp::LBool(false)
        )
    )
);

// risp_bool
named!(
    risp_bool<CompleteStr, Risp>,
    alt!(risp_true | risp_false)
);

// risp_sexpr
named!(
    risp_sexpr<CompleteStr, Risp>,
    do_parse!(
        terminated!(tag!("("), opt!(multispace)) >>
        list: separated_list!(multispace, risp_val) >>
        preceded!(opt!(multispace), tag!(")")) >>
        (
            Risp::Sexpr(list)
        )
    )
);

// risp_qexpr
named!(
    risp_qexpr<CompleteStr, Risp>,
    do_parse!(
        terminated!(tag!("{"), opt!(multispace)) >>
        list: separated_list!(multispace, risp_val) >>
        preceded!(opt!(multispace), tag!("}")) >>
        (
            Risp::Qexpr(list)
        )
    )
);

// risp_val
named!(
    risp_val<CompleteStr, Risp>,
    alt!( risp_sexpr | risp_qexpr | risp_int_vec_literal | risp_float | risp_integer | risp_bool | risp_prelude | risp_symbol | risp_string | risp_comment | risp_char)
);

named!(risp_int_vec_literal<CompleteStr, Risp>,
    do_parse!(
        terminated!(tag!("#["), opt!(multispace)) >>
        list: separated_list!(multispace, number) >>
        preceded!(opt!(multispace), tag!("]")) >>
        (
            Risp::LVec(TypedVec::NumVec(list))
        )
    )
);
pub fn parse_risp(input: &str) -> Option<Risp> {
    let val = risp_val(CompleteStr(input));
    // println!("Got parse: {:?}", val);
    match val {
        Ok(v) => {
            let (rest, result) = v;
            if rest == CompleteStr("") {
                Some(result)
            } else {
                Some(Risp::LSyntaxErr(format!(
                    "Parse error, unexpected '{}' from '{}'",
                    rest, input
                )))
            }
        }
        Err(e) => Some(Risp::LSyntaxErr(format!("{:?}", e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_multiline() {
        let expression = "(+ 1 2 \n
                            (+ 3 4))";

        let result = parse_risp(expression);
        match result {
            None => panic!("parse error"),
            _ => {}
        }
    }
}

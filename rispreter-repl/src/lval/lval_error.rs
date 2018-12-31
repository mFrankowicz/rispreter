use crate::lval::lval_def::{Lval, LvalType};
use crate::lval::lval_lambda::LLambda;

#[derive(PartialEq, Clone)]
pub enum Lerror {
    GenericError {
        msg: String,
    },
    EmptyList {
        lval: Box<Lval>,
    },
    DivisionByZero,
    CantCompare {
        left: Box<LvalType>,
        right: Box<LvalType>,
    },
    FirstArgumentDoesNotEvalTo {
        expect: LvalTypeMeta,
        got: Box<LvalType>,
    },
    SymbolNotBinded {
        sym: String,
    },
    InvalidOperand {
        op: String,
    },
    WrongNumberOfArgs {
        lval: Box<Lval>,
        expect: usize,
        got: usize,
    },
    LambdaWrongNumberOfArgs {
        llambda: Box<LLambda>,
        expect: usize,
        got: usize,
    },
    LambdaWrongGenericError {
        llambda: Box<LLambda>,
        msg: String,
    },
    IncompatibleNumberOfArgs {
        lval_left: Box<Lval>,
        expect_left: usize,
        expect_right: usize,
        lval_right: Box<Lval>,
        got_left: usize,
        got_right: usize,
    },
    WrongType {
        lval: Box<Lval>,
        expect: LvalTypeMeta,
        got: Box<LvalType>,
    },
}

impl std::fmt::Display for Lerror {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Lerror::GenericError {msg} => writeln!(f,"{}", msg),
            Lerror::EmptyList{lval} => writeln!(f, "Got empty Q-expression at: {:?}", lval),
            Lerror::DivisionByZero => writeln!(f, "Divizion by zero"),
            Lerror::CantCompare {left, right} => writeln!(f, "Can't compare left: {:?} with right: {:?}", left, right),
            Lerror::FirstArgumentDoesNotEvalTo{expect, got} => writeln!(f, "First argument dont evaluates to {:?}, got a {:?} instead", got, expect),
            Lerror::SymbolNotBinded{sym} => writeln!(f, "Synbol {} it's not binded in any environment", sym),
            Lerror::InvalidOperand{op} => writeln!(f, "{} is a invalid operand in this context", op),
            Lerror::WrongNumberOfArgs{lval, expect, got} => writeln!(f, "Invalid number of operands at {}, got: {}, expect: {}", lval, got, expect),
            Lerror::LambdaWrongNumberOfArgs{llambda, expect, got} => writeln!(f, "Invalid number of operands at Function {:?}, got: {}, expect: {}", llambda, got, expect),
            Lerror::LambdaWrongGenericError{llambda, msg} => writeln!(f, "{}, at: {:?}", msg, llambda),
            Lerror::IncompatibleNumberOfArgs{lval_left, expect_left, expect_right, lval_right, got_left, got_right} =>  {
                writeln!(f, "Left and Right side operands doesn't match.\n got left: {}, got right: {}.\n expect left: {}, expect right: {}.\n within {} at left,  and {} at right",
                got_left, got_right, expect_left, expect_right, lval_left, lval_right )},
            Lerror::WrongType{lval, expect, got} => writeln!(f, "Wrong type at {}, got: {}, expect: {:?}", lval, got, expect)
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum LvalTypeMeta {
    LvalErr,
    LvalNum,
    LvalSym,
    LvalFun,
    LvalLambda,
    LvalString,
    LvalBool,
    LvalSexpr,
    LvalQexpr,
}

impl From<LvalType> for LvalTypeMeta {
    fn from(v: LvalType) -> LvalTypeMeta {
        match v {
            LvalType::LVAL_ERR(_) => LvalTypeMeta::LvalErr,
            LvalType::LVAL_NUM(_) => LvalTypeMeta::LvalNum,
            LvalType::LVAL_SYM(_) => LvalTypeMeta::LvalSym,
            LvalType::LVAL_FUN(_) => LvalTypeMeta::LvalFun,
            LvalType::LVAL_LAMBDA(_) => LvalTypeMeta::LvalLambda,
            LvalType::LVAL_STRING(_) => LvalTypeMeta::LvalString,
            LvalType::LVAL_BOOL(_) => LvalTypeMeta::LvalBool,
            LvalType::LVAL_SEXPR => LvalTypeMeta::LvalSexpr,
            LvalType::LVAL_QEXPR => LvalTypeMeta::LvalQexpr,
        }
    }
}

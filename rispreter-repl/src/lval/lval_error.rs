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

impl std::fmt::Debug for Lerror {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Lerror::GenericError { .. } => write!(f, ""),
            Lerror::EmptyList { .. } => write!(f, "EmptyList"),
            Lerror::DivisionByZero => write!(f, "DivisionByZero"),
            Lerror::CantCompare { .. } => write!(f, "CantCompare"),
            Lerror::FirstArgumentDoesNotEvalTo { .. } => write!(f, "FirstArgumentDoesNotEvalTo"),
            Lerror::SymbolNotBinded { .. } => write!(f, "SymbolNotBinded"),
            Lerror::InvalidOperand { .. } => write!(f, "InvalidOperand"),
            Lerror::WrongNumberOfArgs { .. } => write!(f, "WrongNumberOfArgs"),
            Lerror::LambdaWrongNumberOfArgs { .. } => write!(f, "LambdaWrongNumberOfArgs"),
            Lerror::LambdaWrongGenericError { .. } => write!(f, "LambdaWrongGenericError"),
            Lerror::IncompatibleNumberOfArgs { .. } => write!(f, "IncompatibleNumberOfArgs"),
            Lerror::WrongType { .. } => write!(f, "WrongType"),
        }
    }
}

impl std::fmt::Display for Lerror {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} .. ", self)?;
        match self {
            Lerror::GenericError {msg} => write!(f,"{}", msg),
            Lerror::EmptyList{lval} => write!(f, "Got empty Q-expression at: '{:?}'", lval),
            Lerror::DivisionByZero => write!(f, "Divizion by zero"),
            Lerror::CantCompare {left, right} => write!(f, "Can't compare left: '{:?}' with right: '{:?}'", left, right),
            Lerror::FirstArgumentDoesNotEvalTo{expect, got} => write!(f, "First argument dont evaluates to '{:?}', got a '{:?}' instead", got, expect),
            Lerror::SymbolNotBinded{sym} => write!(f, "Symbol '{}' it's not binded in any environment", sym),
            Lerror::InvalidOperand{op} => write!(f, "'{}' is a invalid operand in this context", op),
            Lerror::WrongNumberOfArgs{lval, expect, got} => write!(f, "Invalid number of operands at '{}', got: '{}', expect: '{}'", lval, got, expect),
            Lerror::LambdaWrongNumberOfArgs{llambda, expect, got} => write!(f, "Invalid number of operands at Function '{:?}', got: '{}', expect: '{}'", llambda, got, expect),
            Lerror::LambdaWrongGenericError{llambda, msg} => write!(f, "'{}', at: '{:?}'", msg, llambda),
            Lerror::IncompatibleNumberOfArgs{lval_left, expect_left, expect_right, lval_right, got_left, got_right} =>  {
                write!(f, "Left and Right side operands doesn't match.\n got left: '{}', got right: '{}'.\n expect left: '{}', expect right: '{}'.\n within '{}' at left, and '{}' at right",
                got_left, got_right, expect_left, expect_right, lval_left, lval_right )},
            Lerror::WrongType{lval, expect, got} => write!(f, "Wrong type at '{}', got: '{}', expect: '{:?}'", lval, got, expect)
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
    LvalChar,
    LvalBool,
    LvalIntVec,
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
            LvalType::LVAL_CHAR(_) => LvalTypeMeta::LvalChar,
            LvalType::LVAL_BOOL(_) => LvalTypeMeta::LvalBool,
            LvalType::LVAL_NUM_VEC(_) => LvalTypeMeta::LvalIntVec,
            LvalType::LVAL_SEXPR => LvalTypeMeta::LvalSexpr,
            LvalType::LVAL_QEXPR => LvalTypeMeta::LvalQexpr,
        }
    }
}

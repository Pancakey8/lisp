use std::any::Any;

use crate::{interpreter::EvalError, lisp::internal::*};

#[derive(Clone)]
pub enum LispFunction {
    Internal {
        name: LispSymbol,
        args: LispList,
        func: fn(&Context, Vec<LispExpr>) -> Result<LispExpr, EvalError>,
    },
    Lisp {
        name: LispSymbol,
        args: LispList,
        body: LispList,
    },
}

impl LispType for LispFunction {
    fn value(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn ltype(&self) -> LispTypeId {
        LispTypeId::Function
    }
}

pub fn lisp_println(context: &Context, args: Vec<LispExpr>) -> Result<LispExpr, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::AAAA);
    }

    let expr = args[0].clone();

    match expr {
        LispExpr::Literal(s) => {
            if s.ltype() != LispTypeId::String {
                return Err(EvalError::AAAA);
            }
            let str = s.value().downcast::<LispString>().unwrap();
            println!("{}", str.value);
        }
        _ => return Err(EvalError::AAAA),
    }

    Ok(LispExpr::Null)
}

pub fn lisp_add(context: &Context, args: Vec<LispExpr>) -> Result<LispExpr, EvalError> {
    if args.len() < 2 {
        return Err(EvalError::AAAA);
    }

    let mut nums: Vec<LispNumber> = Vec::new();
    for arg in &args {
        if let LispExpr::Literal(lit) = arg {
            if lit.ltype() != LispTypeId::Number {
                return Err(EvalError::AAAA);
            }

            nums.push(*lit.value().downcast::<LispNumber>().unwrap());
        } else {
            return Err(EvalError::AAAA);
        }
    }

    let mut sum = LispNumber { value: 0.0 };
    for num in nums {
        sum.value += num.value;
    }

    Ok(LispExpr::Literal(Box::new(sum)))
}

pub fn lisp_to_string(context: &Context, args: Vec<LispExpr>) -> Result<LispExpr, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::AAAA);
    }

    match &args[0] {
        LispExpr::Literal(lit) => match lit.ltype() {
            LispTypeId::Symbol => Ok(LispExpr::Literal(Box::new(LispString {
                value: "'".to_string() + &lit.value().downcast::<LispSymbol>().unwrap().value,
            }))),
            LispTypeId::Number => Ok(LispExpr::Literal(Box::new(LispString {
                value: lit
                    .value()
                    .downcast::<LispNumber>()
                    .unwrap()
                    .value
                    .to_string(),
            }))),
            LispTypeId::List => {
                let mut repr = "(".to_string();
                let list = lit.value().downcast::<LispList>().unwrap().value;
                for expr in list {
                    match lisp_to_string(context, vec![expr]) {
                        Ok(expr) => {
                            if let LispExpr::Literal(lit) = expr {
                                let str = lit.value().downcast::<LispString>().unwrap().value;
                                repr.push_str(str.as_str());
                            }
                        }
                        Err(e) => return Err(e),
                    }
                    repr.push_str(", ");
                }
                // remove last comma
                repr.pop();
                repr.pop();
                repr.push(')');
                Ok(LispExpr::Literal(Box::new(LispString { value: repr })))
            }
            LispTypeId::String => return Ok(LispExpr::Literal(lit.clone())),
            LispTypeId::Function => {
                return Err(EvalError::AAAA);
            }
        },
        LispExpr::Null => Ok(LispExpr::Literal(Box::new(LispString {
            value: "nil".to_string(),
        }))),
    }
}

macro_rules! declare_internal {
    ($func:ident, $name:expr, $($arg:expr),*) => {
        LispFunction::Internal {
            name: LispSymbol { value: $name.to_string() },
            args: LispList { value: vec![$(LispExpr::Literal(Box::new(LispSymbol { value: $arg.to_string() }))),*] },
            func: $func
        }
    };
}

pub fn get_internal_functions() -> Vec<LispFunction> {
    vec![
        declare_internal!(lisp_println, "println", "str"),
        declare_internal!(lisp_add, "+", "num1", "num2", "&rest"),
        declare_internal!(lisp_to_string, "string", "param"),
    ]
}

use std::any::Any;

use crate::ast::{ASTNode, ASTNodeValue};

pub trait LispType: LispTypeBoxClone {
    fn value(&self) -> Box<dyn Any>;
    fn ltype(&self) -> LispTypeId;
}

#[derive(PartialEq, Eq)]
pub enum LispTypeId {
    Number,
    String,
    List,
    Symbol,
    Function,
}

#[derive(Clone)]
pub struct LispString {
    value: String,
}

impl LispType for LispString {
    fn value(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn ltype(&self) -> LispTypeId {
        LispTypeId::String
    }
}

#[derive(Clone)]
pub struct LispSymbol {
    value: String,
}

impl LispType for LispSymbol {
    fn value(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn ltype(&self) -> LispTypeId {
        LispTypeId::Symbol
    }
}

#[derive(Clone)]
pub struct LispNumber {
    value: f32,
}

impl LispType for LispNumber {
    fn value(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn ltype(&self) -> LispTypeId {
        LispTypeId::Number
    }
}

#[derive(Clone)]
pub struct LispList {
    value: Vec<LispExpr>,
}

impl LispType for LispList {
    fn value(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn ltype(&self) -> LispTypeId {
        LispTypeId::List
    }
}

#[derive(Clone)]
pub struct LispFunction {
    name: LispSymbol,
    args: LispList,
    body: LispList,
    internal: Option<fn(&Context, Vec<LispExpr>) -> Result<LispExpr, EvalError>>,
}

impl LispType for LispFunction {
    fn value(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn ltype(&self) -> LispTypeId {
        LispTypeId::Function
    }
}

trait LispTypeBoxClone {
    fn box_clone(&self) -> Box<dyn LispType>;
}

impl<T> LispTypeBoxClone for T
where
    T: 'static + LispType + Clone,
{
    fn box_clone(&self) -> Box<dyn LispType> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn LispType> {
    fn clone(&self) -> Box<dyn LispType> {
        self.box_clone()
    }
}

#[derive(Clone)]
pub enum LispExpr {
    Literal(Box<dyn LispType>),
    Null,
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

pub struct Context {
    functions: Vec<LispFunction>,
}

impl Context {
    pub fn default() -> Context {
        Context {
            functions: vec![LispFunction {
                name: LispSymbol {
                    value: "println".to_string(),
                },
                body: LispList { value: Vec::new() },
                args: LispList {
                    value: vec![LispExpr::Literal(Box::new(LispSymbol {
                        value: "string".to_string(),
                    }))],
                },
                internal: Some(lisp_println),
            }],
        }
    }
}

pub struct Evaluator {
    input: Vec<ASTNode>,
    context: Context,
}

#[derive(Debug)]
pub enum EvalError {
    AAAA,
}

impl Evaluator {
    pub fn new(input: Vec<ASTNode>) -> Evaluator {
        Evaluator {
            input,
            context: Context::default(),
        }
    }

    pub fn peek(&self) -> Option<ASTNode> {
        self.input.iter().next().map(|n| n.clone())
    }

    pub fn next(&mut self) -> Option<ASTNode> {
        if self.input.is_empty() {
            None
        } else {
            Some(self.input.remove(0))
        }
    }

    pub fn try_interpret_next(&mut self) -> Result<LispExpr, EvalError> {
        if let Some(node) = self.next() {
            match node.value {
                ASTNodeValue::String(s) => {
                    return Ok(LispExpr::Literal(Box::new(LispString { value: s.clone() })));
                }
                ASTNodeValue::Number(n) => {
                    return Ok(LispExpr::Literal(Box::new(LispNumber { value: n })));
                }
                ASTNodeValue::List(l) => {
                    let mut local_eval = Evaluator::new(l);
                    let mut args = Vec::new();
                    while let Some(n) = local_eval.peek() {
                        match local_eval.try_interpret_next() {
                            Ok(expr) => {
                                args.push(expr);
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    return Ok(LispExpr::Literal(Box::new(LispList { value: args })));
                }
                ASTNodeValue::Ident(s) => {
                    return Ok(LispExpr::Literal(Box::new(LispSymbol { value: s.clone() })));
                }
                _ => {}
            }
        }

        Err(EvalError::AAAA)
    }

    pub fn evaluate_expr(&self, expr: LispExpr) -> Result<LispExpr, EvalError> {
        match expr {
            LispExpr::Literal(lit) => match lit.ltype() {
                LispTypeId::List => {
                    let val = lit.value();
                    let mut list = val.downcast::<LispList>().unwrap().value.clone();

                    if list.len() == 0 {
                        Ok(LispExpr::Literal(lit))
                    } else {
                        if let LispExpr::Literal(clit) = list.remove(0) {
                            let callee = clit.value().downcast::<LispSymbol>().unwrap();
                            for func in &self.context.functions {
                                if func.name.value == callee.value {
                                    match func.internal {
                                        Some(f) => return f(&self.context, list),
                                        None => {
                                            return self.evaluate_expr(LispExpr::Literal(
                                                Box::new(func.body.clone()),
                                            ));
                                        }
                                    }
                                }
                            }
                        }

                        Ok(LispExpr::Null)
                    }
                }
                _ => Ok(LispExpr::Literal(lit)),
            },
            _ => Ok(LispExpr::Null),
        }
    }
}

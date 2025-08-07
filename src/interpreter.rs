use crate::ast::{ASTNode, ASTNodeValue};
use crate::lisp::function::LispFunction;
use crate::lisp::internal::*;
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
                    while let Some(_) = local_eval.peek() {
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
                                match func {
                                    LispFunction::Internal {
                                        name,
                                        args: _,
                                        func,
                                    } => {
                                        if name.value == callee.value {
                                            let mut args_eval = Vec::new();
                                            for expr in list {
                                                match self.evaluate_expr(expr) {
                                                    Ok(expr) => args_eval.push(expr),
                                                    Err(e) => return Err(e),
                                                }
                                            }
                                            return func(&self.context, args_eval);
                                        }
                                    }
                                    LispFunction::Lisp {
                                        name,
                                        args: _,
                                        body: _,
                                    } => {
                                        if name.value == callee.value {
                                            todo!("Have to handle call stack here somehow");
                                        }
                                    }
                                }
                            }
                        }

                        Ok(LispExpr::Null)
                    }
                }
                LispTypeId::Symbol => {
                    let val = lit.value();
                    let sym = val.downcast::<LispSymbol>().unwrap().value.clone();
                    for var in &self.context.variables {
                        if var.name == sym {
                            return Ok(var.value.clone());
                        }
                    }
                    Err(EvalError::AAAA)
                }
                _ => Ok(LispExpr::Literal(lit)),
            },
            _ => Ok(LispExpr::Null),
        }
    }

    pub fn run(&mut self) -> Result<(), EvalError> {
        while let Some(_) = self.peek() {
            match self.try_interpret_next() {
                Ok(expr) => match self.evaluate_expr(expr) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

use std::any::Any;

use crate::lisp::function::*;

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
    pub value: String,
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
    pub value: String,
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
    pub value: f32,
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
    pub value: Vec<LispExpr>,
}

impl LispType for LispList {
    fn value(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn ltype(&self) -> LispTypeId {
        LispTypeId::List
    }
}

pub trait LispTypeBoxClone {
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

pub struct LispVariable {
    pub name: String,
    pub value: LispExpr,
}

pub struct Context {
    pub functions: Vec<LispFunction>,
    pub variables: Vec<LispVariable>,
}

impl Context {
    pub fn default() -> Context {
        Context {
            functions: get_internal_functions(),
            variables: vec![LispVariable {
                name: "nil".to_string(),
                value: LispExpr::Null,
            }],
        }
    }
}

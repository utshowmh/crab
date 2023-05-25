use std::fmt::{Display, Formatter, Result};

type Number = i32;
type Boolean = bool;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Type {
    Number,
    Boolean,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Type::Number => write!(f, "{self:?}"),
            Type::Boolean => write!(f, "{self:?}"),
        }
    }
}

impl Type {
    pub(crate) fn default(&self) -> Object {
        match self {
            Type::Number => Object::Number(0),
            Type::Boolean => Object::Boolean(false),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Number(Number),
    Boolean(Boolean),
}

impl Object {
    pub(crate) fn get_type(&self) -> Type {
        match self {
            Object::Number(_) => Type::Number,
            Object::Boolean(_) => Type::Boolean,
        }
    }

    pub(crate) fn as_number(&self) -> Number {
        match self {
            Object::Number(n) => *n,
            Object::Boolean(b) => panic!("Can not convert {b} to {}", Type::Number),
        }
    }

    pub(crate) fn as_boolean(&self) -> Boolean {
        match self {
            Object::Number(n) => panic!("Can not convert {n} to {}", Type::Boolean),
            Object::Boolean(b) => *b,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Object::Number(n) => write!(f, "{n}"),
            Object::Boolean(b) => write!(f, "{b}"),
        }
    }
}

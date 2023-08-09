use std::fmt::{Display, Formatter, Result};

type Number = i32;
type Boolean = bool;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Unit,
    Number,
    Boolean,
    String,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Type::Unit => write!(f, "{self:?}"),
            Type::Number => write!(f, "{self:?}"),
            Type::Boolean => write!(f, "{self:?}"),
            Type::String => write!(f, "{self:?}"),
        }
    }
}

// impl Type {
//     pub(crate) fn default(&self) -> Object {
//         match self {
//             Type::Unit => Object::Unit,
//             Type::Number => Object::Number(0),
//             Type::Boolean => Object::Boolean(false),
//              Type::String => Object::String(String::from("")),
//         }
//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Unit,
    Number(Number),
    Boolean(Boolean),
    String(String),
}

impl Object {
    pub(crate) fn get_type(&self) -> Type {
        match self {
            Object::Unit => Type::Unit,
            Object::Number(_) => Type::Number,
            Object::Boolean(_) => Type::Boolean,
            Object::String(_) => Type::String,
        }
    }

    pub fn as_number(&self) -> Number {
        match self {
            Object::Number(n) => *n,
            o => panic!("Can not convert {o} to {}", Type::Number),
        }
    }

    pub fn as_boolean(&self) -> Boolean {
        match self {
            Object::Boolean(b) => *b,
            o => panic!("Can not convert {o} to {}", Type::Boolean),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Object::Unit => write!(f, "()"),
            Object::Number(n) => write!(f, "{n}"),
            Object::Boolean(b) => write!(f, "{b}"),
            Object::String(s) => write!(f, "{s}"),
        }
    }
}

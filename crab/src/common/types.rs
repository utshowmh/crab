use std::fmt::Display;

type Number = i32;
type Boolean = bool;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Type {
    Number,
    Boolean,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Number => write!(f, "{self:?}"),
            Type::Boolean => write!(f, "{self:?}"),
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
            Object::Boolean(b) => panic!("Can not convert {b} to {:?}", Type::Number),
        }
    }

    pub(crate) fn as_boolean(&self) -> Boolean {
        match self {
            Object::Number(n) => panic!("Can not convert {n} to {:?}", Type::Boolean),
            Object::Boolean(b) => *b,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Number(n) => write!(f, "{n}"),
            Object::Boolean(b) => write!(f, "{b}"),
        }
    }
}

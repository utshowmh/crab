use std::fmt::Display;

#[derive(Debug)]
pub enum Type {
    Number,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Number => write!(f, "{self:?}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Object {
    Number(i32),
}

impl Object {
    pub(crate) fn get_type(&self) -> Type {
        match self {
            Object::Number(_) => Type::Number,
        }
    }
}

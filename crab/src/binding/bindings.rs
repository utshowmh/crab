use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::common::types::Object;

#[derive(Debug)]
pub struct Bindings {
    pub(crate) outer: Option<Rc<RefCell<Bindings>>>,
    bindings: HashMap<String, Object>,
}

impl Bindings {
    pub fn new() -> Self {
        Self {
            outer: None,
            bindings: HashMap::new(),
        }
    }

    pub fn extend(with: Rc<RefCell<Bindings>>) -> Self {
        Self {
            outer: Some(with),
            bindings: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, name: &str) -> Option<Object> {
        if let Some(object) = self.bindings.get(name) {
            Some(object.clone())
        } else if let Some(outer) = &self.outer {
            outer.borrow().get(name)
        } else {
            None
        }
    }

    pub(crate) fn reset(&mut self, name: String, object: Object) {
        if let Some(_) = self.bindings.get(&name) {
            self.bindings.insert(name, object);
        } else if let Some(outer) = &self.outer {
            outer.borrow_mut().reset(name, object)
        } else {
            return;
        }
    }

    pub(crate) fn set(&mut self, name: String, object: Object) {
        self.bindings.insert(name, object);
    }
}
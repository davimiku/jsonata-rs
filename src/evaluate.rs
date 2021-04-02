//! Execution of the Abstract Syntax Tree (AST)

use std::collections::HashMap;

use serde_json::Value;

/// AST nodes implement this trait to be run by the interpreter
pub trait Evaluatable {
    /// Runs this AST node
    fn evaluate(&self, context: &mut Context) -> EvaluatableResult;
}

pub type EvaluatableResult = Result<Option<Value>, EvaluationError>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum EvaluationError {
    NotImplemented, // FIXME: implement
}

#[derive(Debug)]
pub struct Context {
    data: Value,

    variables: HashMap<String, Value>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            data: Value::Null,
            variables: HashMap::new(),
        }
    }
}

// impl Default for Context {
//     fn default() -> Self {

//     }
// }

impl Context {
    pub fn data(&self) -> &Value {
        &self.data
    }

    pub fn set_data(&mut self, data: Value) {
        self.data = data
    }
    // pub(crate) fn create_function<P, B>(
    //     &mut self,
    //     params: P,
    //     body: B
    // ) -> Result<(), ()>
    // where
    //     P: Into<Box<[FormalParameter]>>,
    //     B: Into<StatementList>,
    //     {}
}

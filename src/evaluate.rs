//! Execution of the Abstract Syntax Tree (AST)

use std::collections::HashMap;

use serde_json::Value;

/// AST nodes implement this trait to be run by the interpreter
pub trait Evaluatable {
    /// Runs this AST node
    fn evaluate(&self, context: &mut Context) -> EvaluationResult;
}

pub type EvaluationResult = Result<Option<Value>, EvaluationError>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum EvaluationError {
    NotImplemented, // FIXME: implement
}

#[derive(Debug)]
pub struct Context {
    data: Value,

    variables: HashMap<String, Option<Value>>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            data: Value::Null,
            variables: HashMap::new(),
        }
    }
}

impl Context {
    pub fn data(&self) -> &Value {
        &self.data
    }

    pub fn set_var(&mut self, var_name: String, value: Option<Value>) {
        self.variables.insert(var_name, value);
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

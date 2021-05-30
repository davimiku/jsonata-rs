//! Execution of the Abstract Syntax Tree (AST)

use std::collections::HashMap;

use serde_json::Value;

pub type EvaluationResult = Result<Option<Value>, EvaluationError>;

#[derive(PartialEq, Clone, Debug)]
pub enum EvaluationError {
    /// The values '{}' and '{}' on either side of operator '{}' must be of the same data type
    DyadicInconsistentDataType(Value, Value, String),

    /// The expressions on either side of operator '{}' must evaluate to numeric values
    DyadicMustBeNumber(String),

    /// The expressions on either side of operator '{}' must evaluate to numeric or string values
    DyadicMustBeNumberOrString(String),
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

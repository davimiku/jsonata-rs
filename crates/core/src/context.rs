use crate::value::{JSONValue, JSONataVariables};

pub struct Context {
    variables: JSONataVariables,

    pub(crate) data: JSONValue,

    parent: Option<Box<Context>>,
}

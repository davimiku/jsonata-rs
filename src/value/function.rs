use std::{fmt, rc::Rc};

use crate::evaluate::EvaluationResult;

use super::JSONataValue;

pub struct JSONataFunction {
    /// Function which takes a slice of JSONataValue's as arguments
    /// and returns a EvaluationResult
    pub(super) func: Rc<dyn Fn(&[Option<JSONataValue>]) -> EvaluationResult>,

    /// Identifier for the function without the preceding `$` symbol. For example,
    /// the built-in function $max has an ident of "max". Two functions may not have
    /// the same ident.
    pub(super) ident: String,

    /// A function signature is a string of the form <params:return>. params is a sequence of type symbols, each one representing an input argument's type. return is a single type symbol representing the return value type.
    ///
    /// Type symbols work as follows:
    ///
    /// Simple types:
    ///
    /// * b - Boolean
    /// * n - number
    /// * s - string
    /// * l - null
    ///
    /// Complex types:
    ///
    /// * a - array
    /// * o - object
    /// * f - function
    ///
    /// Union types:
    ///
    /// * (sao) - string, array or object
    /// * (o) - same as o
    /// * u - equivalent to (bnsl) i.e. Boolean, number, string or null
    /// * j - any JSON type. Equivalent to (bnsloa) i.e. Boolean, number, string, null, object or array, but not function
    /// * x - any type. Equivalent to (bnsloaf)
    ///
    /// Parameterized types:
    ///
    /// * a<s> - array of strings
    /// * a<x> - array of values of any type
    ///
    /// Some examples of signatures of built-in JSONata functions:
    ///
    /// * $count has signature <a:n>; it accepts an array and returns a number.
    /// * $append has signature <aa:a>; it accepts two arrays and returns an array.
    /// * $sum has signature <a<n>:n>; it accepts an array of numbers and returns a number.
    /// * $reduce has signature <fa<j>:j>; it accepts a reducer function f and an a<j> (array of JSON objects) and returns a JSON object.
    ///
    /// Each type symbol may also have options applied.
    ///
    /// * + : one or more arguments of this type
    ///         E.g. $zip has signature <a+>; it accepts one array, or two arrays, or three arrays, or...
    /// * ? : optional argument
    ///         E.g. $join has signature <a<s>s?:s>; it accepts an array of strings and an optional joiner string which defaults to the empty string. It returns a string.
    /// * - : if this argument is missing, use the context value ("focus").
    ///         E.g. $length has signature <s-:n>; it can be called as $length(OrderID) (one argument) but equivalently as OrderID.$length().
    pub(super) signature: String,
}

impl JSONataFunction {
    fn test(&self) -> EvaluationResult {
        (self.func)(&[])
    }

    pub fn ident(&self) -> &str {
        &self.ident
    }
}

impl fmt::Debug for JSONataFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "todo: format function")
    }
}

impl fmt::Display for JSONataFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "todo: format function")
    }
}

impl From<JSONataFunction> for JSONataValue {
    fn from(func: JSONataFunction) -> Self {
        JSONataValue::Function(func)
    }
}

impl PartialEq for JSONataFunction {
    fn eq(&self, other: &Self) -> bool {
        self.ident() == other.ident()
    }
}

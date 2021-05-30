use serde_json::Value;

use super::BuiltIns;

impl BuiltIns {
    /// Returns an array containing the keys in the object. If the argument
    /// is an array of objects, then the array returned contains a
    /// de-duplicated list of all the keys in all of the objects.
    pub(crate) fn keys(val: Option<Value>) -> Option<Value> {
        todo!()
    }

    /// Returns the value associated with `key` in `object`. If the first argument is
    /// an array of objects, then all of the objects in the array are searched,
    /// and the values associated with all occurrences of `key` are returned.
    pub(crate) fn lookup(object: Option<Value>, key: String) -> Option<Value> {
        todo!()
    }

    /// Splits an `object` containing key/value pairs into an array of objects,
    /// each of which has a single key/value pair from the input `object`.
    /// If the parameter is an array of objects, then the resultant array contains
    /// an object for every key/value pair in every object in the supplied array.
    pub(crate) fn spread(object: Option<Value>) -> Option<Value> {
        todo!()
    }

    /// Merges an array of objects into a single object containing all the key/value pairs
    /// from each of the objects in the input array. If any of the input objects contain
    /// the same key, then the returned object will contain the value of the last one in
    /// the array. It is an error if the input array contains an item that is not an object.
    ///
    /// TODO: This applies to arrays, should probably go under the sequence builtins instead?
    pub(crate) fn merge(object: Option<Value>) -> Option<Value> {
        todo!()
    }

    /// Returns an array containing the values return by the function when applied to
    /// each key/value pair in the object.
    ///
    /// The function parameter will get invoked with two arguments:
    ///
    /// function(value, name)
    ///
    /// where the value parameter is the value of each name/value pair in the object and
    /// name is its name. The name parameter is optional.
    ///
    /// ## Example
    ///
    /// ```
    /// $each(Address, function($v, $k) {$k & ": " & $v})
    /// ```
    /// Results in:
    /// ```
    /// [
    ///   "Street: Hursley Park",
    ///   "City: Winchester",
    ///   "Postcode: SO21 2JN"
    /// ]
    /// ```
    pub(crate) fn each(object: Option<Value>, function: ()) -> Option<Value> {
        todo!()
    }

    /// Deliberately throws an error with an optional `message`
    ///
    /// TODO: How to handle this best? This probably should be handled a layer
    /// above this module
    pub(crate) fn error(message: Option<String>) {
        todo!()
    }

    /// If condition is true, the function returns undefined. If the condition
    /// is false, an exception is thrown with the message as the message of the exception.
    ///
    /// TODO: Again, decide how to handle this error-wise, this layer of the module
    /// assumes no errors. Seems like it's basically calling the $boolean built-in to evaluate
    ///
    /// TODO: Is message optional? In JSONata exerciser, not passing message results in error
    /// "$assert() statement failed"
    /// which seems like a caught exception, I think we'd rather check the arguments up-front
    pub(crate) fn assert(condition: (), message: String) {
        todo!()
    }

    /// Evaluates the type of value and returns one of the following strings:
    ///
    /// - "null"
    /// - "number"
    /// - "string"
    /// - "boolean"
    /// - "array"
    /// - "object"
    /// - "function"
    /// - None if input value is None
    /// FIXME: Add functions
    pub(crate) fn r#type(val: Option<Value>) -> Option<Value> {
        if let Some(val) = val {
            Some(Value::String(
                match val {
                    Value::Null => "null",
                    Value::Bool(_) => "boolean",
                    Value::Number(_) => "number",
                    Value::String(_) => "string",
                    Value::Array(_) => "array",
                    Value::Object(_) => "object",
                }
                .to_string(),
            ))
        } else {
            None
        }
    }
}

use serde_json::Value;

// use rand::seq::SliceRandom;
// use rand::thread_rng;

use crate::{evaluate::EvaluationResult, value::JSONataValue};

#[cfg(test)]
mod tests;

use super::BuiltIns;

impl BuiltIns {
    /// Ensures that the provided value is turned
    /// into a vec if it is not a Value::Array
    // fn vecify(input: JSONataValue) -> Vec<JSONataValue> {
    //     match input {
    //         JSONataValue::Value(val) => match val {
    //             Value::Array(_) => input,
    //             a => vec![a],
    //         }
    //         .into(),
    //         JSONataValue::Function(func) => vec![func.to_string().into()],
    //     }
    // }

    /// Returns the number of items in the `array` parameter.
    /// If the array parameter is not an `array`, but rather a
    /// value of another JSON type, then the parameter is
    /// treated as a singleton array containing that value,
    /// and this function returns `1`.
    ///
    /// If `array` is not specified, then the context value is used as the value of `array`.
    ///
    /// ### Examples
    ///
    /// ```text
    /// $count([1,2,3,1]) => 4
    /// $count("hello") => 1
    /// ```
    ///
    /// **Signature**: `$count(array)`
    pub(crate) fn count(val: &Option<JSONataValue>) -> EvaluationResult {
        Ok(Some(
            match val {
                Some(val) => match val {
                    JSONataValue::JSONValue(val) => match &val.0 {
                        Value::Null => 1,
                        Value::Bool(_) => 1,
                        Value::Number(_) => 1,
                        Value::String(_) => 1,
                        Value::Array(v) => v.len(),
                        Value::Object(_) => 1,
                    },
                    JSONataValue::Function(_) => 1,
                },
                None => 0,
            }
            .into(),
        ))
    }

    /// Returns an array containing the values in array1
    /// followed by the values in array2. If either parameter
    /// is not an array, then it is treated as a singleton
    /// array containing that value.
    ///
    /// TODO: The following are implementation notes from try.jsonata.org
    /// but are not documented as behaviors
    ///
    /// - If one argument is None are the other argument is Some, the other
    ///    argument is returned as-is (even for non-array args)
    /// - If both arguments are None, the return value is None.
    pub(crate) fn append(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!();
        // let array1 = args.get(0).unwrap(); // arg will exist
        // let array2 = args.get(1);
        // match (array1, array2) {

        // }
        // if array1.is_none() && array2.is_none() {
        //     return None;
        // }
        // let mut v = BuiltIns::vecify(array1);
        // v.append(&mut BuiltIns::vecify(array2));
        // Some(v.into())
    }

    /// Returns an array containing all the values in the `array` parameter,
    /// but sorted into order. If no `function` parameter is supplied, then the `array`
    /// parameter must contain only numbers or only strings, and they will be sorted in
    /// order of increasing number, or increasing unicode codepoint respectively.
    ///
    /// If a comparator `function` is supplied, then is must be a function that takes two parameters:
    ///
    /// `function(left, right)`
    ///
    /// This function gets invoked by the sorting algorithm to compare two values `left` and `right`.
    /// If the value of `left` should be placed after the value of `right` in the desired sort order,
    /// then the function must return Boolean `true` to indicate a swap. Otherwise it must return `false`.
    ///
    /// ## Example
    ///
    /// ```text
    /// $sort(Account.Order.Product, function($l, $r) {
    ///   $l.Description.Weight > $r.Description.Weight
    /// })
    /// ```
    ///
    /// This sorts the products in order of increasing weight.
    ///
    /// The sorting algorithm is stable which means that values within the original array which are
    /// the same according to the comparator function will remain in the original order in the sorted array.
    /// FIXME: Come back to this when functions are implemented
    pub(crate) fn sort(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Returns an array containing all the values from the array parameter, but in reverse order.
    ///
    /// ##Examples
    ///
    /// ```text
    /// $reverse(["Hello", "World"]) => ["World", "Hello"]
    /// [1..5] ~> $reverse() => [5, 4, 3, 2, 1]
    /// ```
    /// TODO: How does this work for non array values?
    pub(crate) fn reverse(args: &[Option<JSONataValue>]) -> EvaluationResult {
        let val = args.get(0).unwrap(); // arg will exist
        if let Some(val) = val {
            match val {
                JSONataValue::JSONValue(_) => todo!(),
                JSONataValue::Function(_) => todo!(),
                // Value::Array(mut vec) => {
                //     vec.reverse();
                //     vec
                // }
                // v => vec![v],
            }
        } else {
            Ok(None)
        }
    }

    /// Returns an array containing all the values from the array parameter,
    /// but shuffled into random order.
    // pub(crate) fn shuffle(args: &[Option<JSONataValue>]) -> EvaluationResult {
    //     let arr = args.get(0).unwrap(); // arg will exist
    //     if let Some(arr) = arr {
    //         match arr {
    //             JSONataValue::JSONValue(_) => todo!(),
    //             JSONataValue::Function(f) => todo!(),
    //         }
    //     }
    //     let mut vec: Vec<u32> = (0..10).collect();
    //     vec.shuffle(&mut thread_rng());
    //     todo!()
    // }

    /// Returns an array containing all the values from the array parameter, but
    /// with any duplicates removed. Values are tested for deep equality as if by
    /// using the equality operator.
    ///
    /// ## Examples
    ///
    /// ```text
    /// $distinct([1,2,3,3,4,3,5]) => [1, 2, 3, 4, 5]
    /// $distinct(Account.Order.Product.Description.Colour) => [ "Purple", "Orange", "Black" ]
    /// ```
    pub(super) fn distinct(array: &JSONataValue) -> EvaluationResult {
        todo!()
    }

    /// Returns a convolved (zipped) array containing grouped arrays of values from
    /// the array1 ... arrayN arguments from index 0, 1, 2, etc.
    ///
    /// This function accepts a variable number of arguments. The length of the returned
    /// array is equal to the length of the shortest array in the arguments.
    ///
    /// ## Examples
    ///
    /// ```text
    /// $zip([1,2,3], [4,5,6]) => [[1,4] ,[2,5], [3,6]]
    /// $zip([1,2,3],[4,5],[7,8,9]) => [[1,4,7], [2,5,8]]
    /// ```
    pub(crate) fn zip(arrays: &[Value]) {
        todo!()
    }

    /// Returns the arithmetic sum of an array of numbers.
    ///
    /// It is an error if the input array contains an item which isn't a number.
    pub(crate) fn sum(array: Value) -> EvaluationResult {
        // TODO: understand how try_fold works to return an error
        todo!()
        // match array {
        //     Value::Number(_) => Ok(Some(array)),
        //     Value::Array(v) => v
        //         .iter()
        //         .try_fold(JSONataNumber::default(), |acc, &val| match val {
        //             Value::Number(n) => {
        //                 let j_num = JSONataNumber::from(n);
        //                 acc + j_num
        //             }
        //             _ => Err(EvaluationError::FunctionInvalidArgument(
        //                 1,
        //                 "sum".to_string(),
        //                 "an array of numbers".to_string(),
        //             )),
        //         }),
        //     _ => Err(EvaluationError::FunctionInvalidArgument(
        //         1,
        //         "sum".to_string(),
        //         "an array of numbers".to_string(),
        //     )),
        // }
    }

    /// Returns the maximum number in an array of numbers. It is an error if the
    /// input array contains an item which isn't a number.
    ///
    /// ## Example
    ///
    /// ```text
    /// $max([5,1,3,7,4]) => 7
    /// ```
    pub(crate) fn max(array: Value) {
        todo!()
    }

    /// Returns the minimum number in an array of numbers. It is an error if the
    /// input array contains an item which isn't a number.
    ///
    /// ## Example
    ///
    /// ```text
    /// $min([5,1,3,7,4]) => 1
    /// ```
    pub(crate) fn min(array: Value) {
        todo!()
    }

    /// Returns the mean number in an array of numbers. It is an error if the
    /// input array contains an item which isn't a number.
    ///
    /// ## Example
    ///
    /// ```text
    /// $average([5,1,3,7,4]) => 4
    /// ```
    pub(crate) fn average(array: Value) {
        todo!()
    }
}

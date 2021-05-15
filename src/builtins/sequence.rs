use serde_json::Value;

/// Ensures that the provided value is turned
/// into a vec if it is not a Value::Array
fn vecify(input: Option<Value>) -> Vec<Value> {
    if let Some(val) = input {
        match val {
            Value::Null => vec![Value::Null],
            Value::Bool(b) => vec![Value::Bool(b)],
            Value::Number(n) => vec![Value::Number(n)],
            Value::String(s) => vec![Value::String(s)],
            Value::Array(v) => v,
            Value::Object(o) => vec![Value::Object(o)],
        }
    } else {
        Vec::new()
    }
}

/// Returns the number of items in the array parameter.
/// If the array parameter is not an array, but rather a
/// value of another JSON type, then the parameter is
/// treated as a singleton array containing that value,
/// and this function returns 1.
pub(super) fn count(array: Option<Value>) -> Value {
    if let Some(val) = array {
        match val {
            Value::Null => 1,
            Value::Bool(_) => 1,
            Value::Number(_) => 1,
            Value::String(_) => 1,
            Value::Array(v) => v.len(),
            Value::Object(_) => 1,
        }
    } else {
        0
    }
    .into()
}

/// Returns an array containing the values in array1
/// followed by the values in array2. If either parameter
/// is not an array, then it is treated as a singleton
/// array containing that value.
///
/// If one of the arguments is None, it is treated as an
/// empty array and
///
/// If both arguments are None, the return value is None.
pub(super) fn append(array1: Option<Value>, array2: Option<Value>) -> Option<Value> {
    if array1.is_none() && array2.is_none() {
        return None;
    }
    let mut v = vecify(array1);
    v.append(&mut vecify(array2));
    Some(v.into())
}

/// Returns an array containing all the values in the array parameter, but sorted into order. If no function parameter is supplied, then the array parameter must contain only numbers or only strings, and they will be sorted in order of increasing number, or increasing unicode codepoint respectively.
///
/// If a comparator function is supplied, then is must be a function that takes two parameters:
///
/// function(left, right)
///
/// This function gets invoked by the sorting algorithm to compare two values left and right. If the value of left should be placed after the value of right in the desired sort order, then the function must return Boolean true to indicate a swap. Otherwise it must return false.
///
/// Example
///
/// $sort(Account.Order.Product, function($l, $r) {
///   $l.Description.Weight > $r.Description.Weight
/// })
///
/// This sorts the products in order of increasing weight.
///
/// The sorting algorithm is stable which means that values within the original array which are the same according to the comparator function will remain in the original order in the sorted array.
// FIXME: Come back to this when functions are implemented
pub(super) fn sort(array: Vec<Value>, func: Option<()>) -> Vec<Value> {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn append_nones() {
        assert!(append(None, None).is_none());
        assert_eq!(
            Some(Value::Array(vec![Value::Null])),
            append(None, Some(vec![Value::Null].into())),
        );
        assert_eq!(
            append(None, Some(vec![Value::Null].into())),
            Some(Value::Array(vec![Value::Null])),
        );
    }

    #[test]
    fn append_somes() {
        assert_eq!(
            Some(Value::Array(vec![true.into(), false.into()])),
            append(
                Some(vec![Value::Bool(true)].into()),
                Some(vec![Value::Bool(false)].into())
            ),
        );
    }
}

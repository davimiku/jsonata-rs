use serde_json::Value;

use crate::evaluate::{Context, EvaluationResult};

use super::expr::Expression;

/*
FIXME:

This probably needs to be refactored to something like:

struct PathExpression {
    pub context: Value,
    pub ident: String,
}

struct MapExpression {
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

Rename Context struct to State, and add a mut `context` member to the struct of type Value.

Map works by evaluating the lhs (and forcing into an array if needed),
or short-circuiting and returning None if lhs evaluates to None (or empty array)

Then map iterates through each value produced from lhs and uses that value as the context for
evaluating rhs. Evaluation of rhs is also coerced into an array, or return None.

Account.Order.OrderID.$uppercase() => [ "ORDER103", "ORDER104"]

Account.Order.Product.(Price * Quantity) => [ 68.9, 21.67, 137.8, 107.99 ]

*/

/// MapExpression is a way to compose several expressions related
/// to paths to get values from the JSON data.
///
/// Example:
/// ```json
/// {
///   "Account": {
///      "Name": "Mike Wazowski"
///    }
/// }
/// ```
///
/// The MapExpression `Account.Name` which can be thought of as
/// `PathExpression(Account) --map--> PathExpression(Name)` would yield
/// the value "Mike Wazowski".
#[derive(PartialEq, Debug)]
pub struct MapExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl MapExpression {
    /// Constructor from Path expressions
    pub fn from_paths(path1: PathExpression, path2: PathExpression) -> Self {
        MapExpression {
            lhs: Box::new(path1.into()),
            rhs: Box::new(path2.into()),
        }
    }

    /// Evaluate a Map expression
    pub fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let lhs = self.lhs.evaluate(context)?;
        if let Some(data) = lhs {
            // TODO: Check if data is an Array, if so, then the RHS would
            // be evaluated for each item in the array.
            // `users.id` where users is an Array
            self.rhs.evaluate(&mut Context::from_data(&data))
        } else {
            Ok(None)
        }
    }
}

/// The filter operator (a.k.a predicate) is used to select only the items
/// in the input sequence that satisfy the predicate expression contained
/// between the square brackets.
///
/// If the predicate expression is an integer, or an expression that evaluates
/// to an integer, then the item at that position (zero offset) in the input
/// sequence is the only item selected for the result sequence. If the number
/// is non-integer, then it is rounded down to the nearest integer.
///
/// If the predicate expression is an array of integers, or an expression that
/// evaluates to an array of integers, then the items at those positions (zero offset)
/// in the input sequence is the only item selected for the result sequence.
///
/// If the predicate expression evaluates to any other value, then it is cast to a Boolean
/// as if using the $boolean() function. If this evaluates to true, then the item is
/// retained in the result sequence. Otherwise it is rejected.
///
/// ## Examples
///
/// ```
/// Phone[type='mobile']  /* Sequence of all Phone objects with a `type` key equal to "mobile" */
/// ```
///
/// ```
/// Phone[3]    /* Singleton sequence of the index-3 Phone */
/// ```
///
/// ```
/// Phone[[0..2]]  /* Sequence of the index-0, index-1, index-2 Phone */
/// ```
#[derive(PartialEq, Debug)]
pub struct FilterExpression {
    pub lhs: Box<Expression>,
    pub pred: Box<Expression>,
}

impl FilterExpression {
    /// Evaluate a Filter expression
    pub fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let lhs = self.lhs.evaluate(context)?;
        if let Some(data) = lhs {
            // lhs evaluation becomes the context for evaluation of the predicate
            let mut new_context = Context::from_data(&data);
            let pred = self.pred.evaluate(&mut new_context)?;
            if let Some(pred_val) = pred {
                // If value is int, then return item at that index of the array
                // or if it's not an array, then return the item itself only if
                // the index is zero  (i.e. treating as a singleton sequence)
                todo!()
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

/// The order-by operator is used to sort an array of values into ascending or descending order
/// according to one or more expressions defined within the parentheses.
///
/// By default, the array will be sorted into ascending order. For example:
///
/// ```
/// Account.Order.Product^(Price)
/// ```
///
/// sorts all of the products into order of increasing price
/// (Price is a numeric field in the Product object).
///
/// To sort in descending order, the sort expression must be preceded by the > symbol. For example:
///
/// ```
/// Account.Order.Product^(>Price)
/// ```
///
/// sorts all of the products into order of decreasing price. The < symbol can be used explicitly
/// indicate ascending order, although that is the default behaviour.
///
/// Secondary (and more) sort expressions can be specified by separating them with commas (,).
/// The secondary expression will be used to determine order if the primary expression ranks
/// two values the same. For example,
///
/// ```
/// Account.Order.Product^(>Price, <Quantity)
/// ```
///
/// orders the products primarily by decreasing price, but for products of the same price,
/// by increasing quantity.
///
/// The sort expression(s) can be any valid JSONata expression that evaluates to a number or a string.
/// If it evaluates to a string then the array is sorted in order of unicode codepoint.
///
/// ## Examples
///
/// ```
/// Account.Order.Product^(Price * Quantity) /* Increasing order of price times quantity. */
/// student[type='fulltime']^(DoB).name /* The names of all full time students sorted by date of birth (the DoB value is an ISO 8601 date format) */
/// ```
#[derive(PartialEq, Debug)]
pub struct OrderByExpression {
    pub lhs: Box<Expression>,
    pub order_by: (), /* potentially introduce OrderByPredicateExpression to be able to parse preceding `>` or `<` */
}

impl OrderByExpression {
    /// Evaluate a OrderByExpression
    pub fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        todo!()
    }
}

/// The reduce operator can be used as the last step in a path expression
/// to group and aggregate its input sequence into a single object.
///
/// The key/value pairs between the curly braces determine the groupings
/// (by evaluating the key expression) and the aggregated values for each group.
///
/// The JSONata object constructor syntax allows you to specify an expression for
/// the key in any key/value pair (the value can obviously be an expression too).
/// The key expression must evaluate to a string since this is a restriction on
/// JSON objects. The key and value expressions are evaluated for each item in the
/// input context (see processing model). The result of each key/value expression
/// pair is inserted into the resulting JSON object.
///
/// If the evaluation of any key expression results in a key that is already in the
/// result object, then the result of its associated value expression will be grouped
/// with the value(s) already associated with that key. Note that the value expressions
/// are not evaluated until all of the grouping has been performed. This allows for
/// aggregation expressions to be evaluated over the collection of items for each group.
///
/// ## Examples
///
/// ```
/// Account.Order.Product{`Product Name`: Price}
/// ```
///
/// ```
/// Account.Order.Product {
///  `Product Name`: {"Price": Price, "Qty": Quantity}
/// }
/// ```
#[derive(PartialEq, Debug)]
pub struct ReduceExpression {
    pub lhs: Box<Expression>,
}

impl ReduceExpression {
    /// Evaluate a ReduceExpression
    pub fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        todo!()
    }
}

/// PathExpression is a way to get a Value from the JSON data
///
/// Example:
/// ```json
/// {
///    "name": "ACME Corp.",
///    "address": {
///      "street": "Main St."
///    }
/// }
/// ```
/// The following raw expressions yield these values:
/// * `name` --> `"ACME Corp."`
/// * `address` --> `{ "street": "Main St." }`
///
/// The PathExpression struct holds the identifier to get from the
/// JSON data.
///
#[derive(PartialEq, Debug)]
pub struct PathExpression {
    pub ident: String,
}

impl PathExpression {
    /// Constructor from a &str
    pub fn from_str(ident: &str) -> Self {
        PathExpression {
            ident: ident.to_string(),
        }
    }

    /// Evaluate a Path expression
    pub fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        Ok(self.get_value(context.data()))
    }
}

impl PathExpression {
    fn get_value(&self, data: &Value) -> Option<Value> {
        if data.is_object() {
            let value = data.get(self.ident.clone())?;
            Some(value.clone())
        } else if let Some(arr) = data.as_array() {
            let values: Vec<Value> = arr.iter().filter_map(|val| self.get_value(val)).collect();
            if values.len() > 0 {
                Some(Value::Array(values))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl From<&str> for PathExpression {
    fn from(s: &str) -> Self {
        PathExpression {
            ident: s.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use serde_json::json;

    use crate::tests::object_data;

    use super::*;

    /// Helper function to get a nested member from the
    /// test data for easier processing.
    fn get_orders() -> Value {
        let data = object_data();
        let path: PathExpression = "orders".into();

        path.get_value(&data).unwrap()
    }

    #[test]
    fn path_get_primitive() {
        let data = object_data();
        let path: PathExpression = "name".into();

        let actual = path.get_value(&data).unwrap();
        let expected = json!("ACME Corp.");

        assert_eq!(actual, expected);
    }

    #[test]
    fn path_get_object() {
        let data = object_data();
        let path: PathExpression = "contact".into();

        let actual = path.get_value(&data).unwrap();
        let expected = json!({ "name": "John Doe" });

        assert_eq!(actual, expected);
    }

    #[test]
    fn path_get_multiple_values_from_array() {
        let orders = get_orders();
        assert!(orders.is_array());

        // Get the ids within the orders
        let ids_path: PathExpression = "id".into();
        let actual = ids_path.get_value(&orders).unwrap();
        let expected = json!([1, 2]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn path_get_nonexisting_from_array() {
        let orders = get_orders();

        let path: PathExpression = "does_not_exist".into();

        let actual = path.get_value(&orders);
        assert!(actual.is_none());
    }

    #[test]
    fn simple_map_expression() {
        let data = object_data();
        let mut context = Context::from_data(&data);
        let map = MapExpression::from_paths("address".into(), "street".into());

        let actual = map.evaluate(&mut context);

        assert_eq!(actual, Ok(Some(json!("Main St."))));
    }

    #[test]
    fn map_with_array_lhs() {
        let data = object_data();
        let mut context = Context::from_data(&data);
        let map = MapExpression::from_paths("orders".into(), "id".into());

        let actual = map.evaluate(&mut context);

        assert_eq!(actual, Ok(Some(json!([1, 2]))));
    }
}

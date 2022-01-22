use serde_json::Value;

use crate::{
    evaluate::{EvaluationError, EvaluationResult},
    value::{number::JSONataNumber, JSONataValue},
};

use super::BuiltIns;

#[cfg(test)]
mod tests;

impl BuiltIns {
    /// Casts the `arg` parameter to a number using the following casting rules
    ///
    /// * Numbers are unchanged
    /// * Strings that contain a sequence of characters that represent a legal JSON number
    ///   are converted to that number
    /// * Boolean `true` casts to `1`, Boolean `false` casts to `0`
    /// * All other values cause an error to be thrown.
    ///
    /// If `arg` is not specified (i.e. this function is invoked with no arguments),
    /// then the context value is used as the value of `arg`.
    ///
    /// ## Examples
    ///
    /// ```
    /// $number("5") => 5
    /// ["1", "2", "3", "4", "5"].$number() => [1, 2, 3, 4, 5]
    /// ```
    /// **Signature**: `$number(arg)`
    pub(super) fn number(args: &[Option<JSONataValue>]) -> EvaluationResult {
        let arg = args.get(0).unwrap();
        todo!()
    }

    /// Returns the absolute value of the `number` parameter, i.e. if the number is negative,
    /// it returns the positive value.
    ///
    /// If `number` is not specified (i.e. this function is invoked with no arguments), then
    /// the context value is used as the value of `number`.
    ///
    /// ## Examples
    ///
    /// ```
    /// $abs(5) => 5
    /// $abs(-5) => -5
    /// ```
    /// **Signature**: `$abs(number)`
    pub(super) fn abs(args: &[Option<JSONataValue>]) -> EvaluationResult {
        let number = args.get(0).unwrap(); // arg will exist
        if let Some(number) = number {
            match number {
                JSONataValue::JSONValue(val) => match val {
                    Value::Number(n) => {
                        let n: JSONataNumber = n.into();
                        Ok(Some(n.abs().into()))
                    }
                    _ => Err(EvaluationError::function_invalid_argument(
                        "abs", 1, "number",
                    )),
                },
                JSONataValue::Function(_) => Err(EvaluationError::function_invalid_argument(
                    "abs", 1, "number",
                )),
            }
        } else {
            Ok(None)
        }
    }

    /// Returns the value of `number` rounded down to the nearest integer that is smaller or
    /// equal to `number`.
    ///
    /// If `number` is not specified (i.e. this function is invoked with no arguments), then
    /// the context value is used as the value of `number`.
    ///
    /// ## Examples
    ///
    /// ```
    /// $floor(5) => 5
    /// $floor(5.3) => 5
    /// $floor(5.8) => 5
    /// $floor(-5.3) => -6
    /// ```
    /// **Signature**: `$floor(number)`
    pub(super) fn floor(args: &[JSONataValue]) -> EvaluationResult {
        let number = args.get(0);
        if let Some(number) = number {
            match number {
                JSONataValue::JSONValue(val) => match val {
                    Value::Number(n) => {
                        let n: JSONataNumber = n.into();
                        Ok(Some(n.floor().into()))
                    }
                    _ => Err(EvaluationError::function_invalid_argument(
                        "floor", 1, "number",
                    )),
                },
                JSONataValue::Function(_) => Err(EvaluationError::function_invalid_argument(
                    "floor", 1, "number",
                )),
            }
        } else {
            Ok(None)
        }
    }

    /// Returns the value of `number` rounded up to the nearest integer that is greater
    /// than or equal to `number`.
    ///
    /// If `number` is not specified (i.e. this function is invoked with no arguments),
    /// then the context value is used as the value of `number`.
    ///
    /// ## Examples
    ///
    /// ```
    /// $ceil(5) => 5
    /// $ceil(5.3) => 6
    /// $ceil(5.8) => 6
    /// $ceil(-5.3) => -5
    /// ```
    /// **Signature**: `$ceil(number)`
    pub(super) fn ceil(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Returns the value of the `number` parameter rounded to the number of decimal places
    /// specified by the optional `precision` parameter.
    ///
    /// The `precision` parameter (which must be an integer) species the number of decimal
    /// places to be present in the rounded number. If `precision` is not specified then it
    /// defaults to the value 0 and the number is rounded to the nearest integer. If `precision`
    /// is negative, then its value specifies which column to round to on the left side of
    /// the decimal place
    ///
    /// This function uses the [Round half to even](https://en.wikipedia.org/wiki/Rounding#Round_half_to_even)
    /// strategy to decide which way to round numbers that fall exactly between two candidates
    /// at the specified precision. This strategy is commonly used in financial calculations and
    /// is the default rounding mode in IEEE 754.
    ///
    /// ## Examples
    ///
    /// ```
    /// $round(123.456) => 123
    /// $round(123.456, 2) => 123.46
    /// $round(123.456, -1) => 120
    /// $round(123.456, -2) => 100
    /// $round(11.5) => 12
    /// $round(12.5) => 12
    /// $round(125, -1) => 120
    /// ```
    /// **Signature**: `$round(number [, precision])`
    pub(super) fn round(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Returns the value of base raised to the power of exponent (baseexponent).
    ///
    /// If base is not specified (i.e. this function is invoked with one argument), then the context value is used as the value of base.
    ///
    /// An error is thrown if the values of base and exponent lead to a value that cannot be represented as a JSON number (e.g. Infinity, complex numbers).
    ///
    /// ## Examples
    ///
    /// ```
    /// $power(2, 8) => 8
    /// $power(2, 0.5) => 1.414213562373
    /// $power(2, -2) => 0.25
    /// ```
    /// **Signature**: `$power(base, exponent)`
    pub(super) fn power(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Returns the square root of the value of the number parameter.
    ///
    /// If number is not specified (i.e. this function is invoked with one argument),
    /// then the context value is used as the value of number.
    ///
    /// An error is thrown if the value of number is negative.
    ///
    /// ## Examples
    ///
    /// ```
    /// $sqrt(4) => 2
    /// $sqrt(2) => 1.414213562373
    /// ```
    /// **Signature**: `$sqrt(number)`
    pub(super) fn sqrt(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Returns a pseudo random number greater than or equal to zero and less than one (0 ≤ n < 1)
    ///
    /// ## Examples
    /// ```
    /// $random() => 0.7973541067127
    /// $random() => 0.4029142127028
    /// $random() => 0.6558078550072
    /// ```
    /// **Signature**: `$random()`
    pub(super) fn random(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Casts the `number` to a string and formats it to a decimal representation as
    /// specified by the `picture` string.
    ///
    /// The behaviour of this function is consistent with the XPath/XQuery function
    /// [fn:format-number](https://www.w3.org/TR/xpath-functions-31/#func-format-number) as
    /// defined in the XPath F&O 3.1 specification. The picture string parameter defines
    /// how the number is formatted and has the
    /// [same syntax](https://www.w3.org/TR/xpath-functions-31/#syntax-of-picture-string)
    /// as fn:format-number.
    ///
    /// The optional third argument `options` is used to override the default locale specific
    /// formatting characters such as the decimal separator. If supplied, this argument must
    /// be an object containing name/value pairs specified in the
    /// [decimal format](https://www.w3.org/TR/xpath-functions-31/#defining-decimal-format)
    /// section of the XPath F&O 3.1 specification.
    ///
    /// ## Examples
    ///
    /// ```
    /// $formatNumber(12345.6, '#,###.00') => "12,345.60"
    /// $formatNumber(1234.5678, "00.000e0") => "12.346e2"
    /// $formatNumber(34.555, "#0.00;(#0.00)") => "34.56"
    /// $formatNumber(-34.555, "#0.00;(#0.00)") => "(34.56)"
    /// $formatNumber(0.14, "01%") => "14%"
    /// $formatNumber(0.14, "###pm", {"per-mille": "pm"}) => "140pm"
    /// $formatNumber(1234.5678, "①①.①①①e①", {"zero-digit": "\u245f"}) => "①②.③④⑥e②"
    /// ```
    /// **Signature**: `$formatNumber(number, picture [, options])`
    pub(super) fn format_number(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Casts the `number` to a string and formats it to an integer represented in the
    /// number base specified by the `radix` argument. If `radix` is not specified, then
    /// it defaults to base 10. `radix` can be between 2 and 36, otherwise an error is thrown.
    ///
    /// ## Examples
    ///
    /// ```
    /// $formatBase(100, 2) => "1100100"
    /// $formatBase(2555, 16) => "9fb"
    /// ```
    /// **Signature**: `$formatBase(number [, radix])`
    pub(super) fn format_base(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Casts the `number` to a string and formats it to an integer representation as
    /// specified by the `picture` string.
    ///
    /// The behaviour of this function is consistent with the two-argument version of the
    /// XPath/XQuery function [fn:format-integer](https://www.w3.org/TR/xpath-functions-31/#func-format-integer)
    /// as defined in the XPath F&O 3.1 specification. The picture string parameter defines
    /// how the number is formatted and has the same syntax as fn:format-integer.
    ///
    /// ## Examples
    ///
    /// ```
    /// $formatInteger(2789, 'w') => "two thousand, seven hundred and eighty-nine"
    /// $formatInteger(1999, 'I') => "MCMXCIX"
    /// ```
    /// **Signature**: `$formatInteger(number, picture)`
    pub(super) fn format_integer(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Parses the contents of the `string` parameter to an integer (as a JSON number)
    /// using the format specified by the `picture` string. The picture string parameter
    /// has the same format as `$formatInteger`. Although the XPath specification does
    /// not have an equivalent function for parsing integers, this capability has been
    /// added to JSONata.
    ///
    /// ## Examples
    ///
    /// ```
    /// $parseInteger("twelve thousand, four hundred and seventy-six", 'w') => 12476
    /// $parseInteger('12,345,678', '#,##0') => 12345678
    /// ```
    /// **Signature**: `$parseInteger(string, picture)`
    pub(super) fn parse_integer(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }
}

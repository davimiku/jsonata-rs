use serde_json::Value;

use crate::{
    evaluate::{EvaluationError, EvaluationResult},
    value::JSONataValue,
};

#[cfg(test)]
mod tests;

use super::BuiltIns;

impl BuiltIns {
    /// Casts the `arg` parameter to a string using the following casting rules
    ///
    /// * Strings are unchanged
    /// * Functions are converted to an empty string
    /// * Numeric infinity and NaN throw an error because they cannot be represented as a JSON number
    /// * All other values are converted to a JSON string using the JSON.stringify function
    ///
    /// If `arg` is not specified (i.e. this function is invoked with no arguments), then the context
    /// value is used as the value of `arg`.
    ///
    /// If `prettify` is true, then "prettified" JSON is produced. i.e One line per field and lines
    /// will be indented based on the field depth.
    ///
    /// ## Examples
    ///
    /// ```
    /// $string(5) => "5"
    /// [1..5].$string() => ["1", "2", "3", "4", "5"]
    /// ```
    /// `Signature: $string(arg, prettify)`
    pub(crate) fn string(args: &[Option<JSONataValue>]) -> EvaluationResult {
        let arg = args.get(0).unwrap(); // arg will exist
        if let Some(arg) = arg {
            Ok(Some(match arg {
                JSONataValue::Value(val) => val.to_string().into(),
                JSONataValue::Function(_) => "".into(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Returns the number of characters in the string str. If str is not specified
    /// (i.e. this function is invoked with no arguments), then the context value is used as
    /// the value of str. An error is thrown if str is not a string.
    ///
    /// ## Examples
    ///
    /// ```
    /// $length("Hello World") => 11
    /// ```
    pub(crate) fn length(args: &[Option<JSONataValue>]) -> EvaluationResult {
        let arg = args.get(0).unwrap(); // arg will exist
        if let Some(arg) = arg {
            if let JSONataValue::Value(val) = arg {
                if let Value::String(s) = val {
                    Ok(Some(s.len().into()))
                } else {
                    Err(EvaluationError::function_invalid_argument(
                        "length", 1, "string",
                    ))
                }
            } else {
                Err(EvaluationError::function_invalid_argument(
                    "length", 1, "string",
                ))
            }
        } else {
            Ok(None)
        }
    }

    /// Returns a string containing the characters in the first parameter str starting at position
    /// start (zero-offset). If str is not specified (i.e. this function is invoked with only the
    /// numeric argument(s)), then the context value is used as the value of str. An error is
    /// thrown if str is not a string.
    ///
    /// If length is specified, then the substring will contain maximum length characters.
    ///
    /// If start is negative then it indicates the number of characters from the end of str.
    /// See substr for full definition.
    ///
    /// ## Examples
    ///
    /// ```
    /// $substring("Hello World", 3) => "lo World"
    /// $substring("Hello World", 3, 5) => "lo Wo"
    /// $substring("Hello World", -4) => "orld"
    /// $substring("Hello World", -4, 2) => "or"
    /// ```
    pub(crate) fn substring(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Returns the substring before the first occurrence of the character sequence chars in str.
    /// If str is not specified (i.e. this function is invoked with only one argument), then the
    /// context value is used as the value of str. If str does not contain chars, then it returns
    /// str. An error is thrown if str and chars are not strings.
    ///
    /// ## Examples
    ///
    /// ```
    /// $substringBefore("Hello World", " ") => "Hello"
    /// ```
    pub(crate) fn substring_before(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Returns the substring after the first occurrence of the character sequence chars in str.
    /// If str is not specified (i.e. this function is invoked with only one argument), then the
    /// context value is used as the value of str. If str does not contain chars, then it returns
    /// str. An error is thrown if str and chars are not strings.
    ///
    /// ## Examples
    ///
    /// ```
    /// $substringAfter("Hello World", " ") => "World"
    /// ```
    pub(crate) fn substring_after(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Returns a string with all the characters of str converted to uppercase. If str is not
    /// specified (i.e. this function is invoked with no arguments), then the context value is
    /// used as the value of str. An error is thrown if str is not a string.
    ///
    /// ## Examples
    ///
    /// ```
    /// $uppercase("Hello World") => "HELLO WORLD"
    /// ```
    pub(crate) fn uppercase(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Returns a string with all the characters of str converted to lowercase. If str is not
    /// specified (i.e. this function is invoked with no arguments), then the context value is
    /// used as the value of str. An error is thrown if str is not a string.
    ///
    /// ## Examples
    ///
    /// ```
    /// $lowercase("Hello World") => "hello world"
    /// ```
    pub(crate) fn lowercase(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Normalizes and trims all whitespace characters in str by applying the following steps:
    ///
    /// 1. All tabs, carriage returns, and line feeds are replaced with spaces.
    /// 2. Contiguous sequences of spaces are reduced to a single space.
    /// 3. Trailing and leading spaces are removed.
    ///
    /// If str is not specified (i.e. this function is invoked with no arguments), then the
    /// context value is used as the value of str. An error is thrown if str is not a string.
    ///
    /// ## Examples
    ///
    /// ```
    /// $trim(" Hello \n World ") => "Hello World"
    /// ```
    pub(crate) fn trim(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Returns a copy of the string str with extra padding, if necessary, so that its
    /// total number of characters is at least the absolute value of the width parameter.
    /// If width is a positive number, then the string is padded to the right; if negative,
    /// it is padded to the left. The optional char argument specifies the padding character(s)
    /// to use. If not specified, it defaults to the space character.
    ///
    /// ## Examples
    ///
    /// ```
    /// $pad("foo", 5) => "foo "
    /// $pad("foo", -5) => " foo"
    /// $pad("foo", -5, "#") => "##foo"
    /// $formatBase(35, 2) ~> $pad(-8, '0') => "00100011"
    /// ```
    pub(crate) fn pad(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Returns true if str is matched by pattern, otherwise it returns false. If str is
    /// not specified (i.e. this function is invoked with one argument), then the context
    /// value is used as the value of str.
    ///
    /// The pattern parameter can either be a string or a regular expression (regex). If
    /// it is a string, the function returns true if the characters within pattern are
    /// contained contiguously within str. If it is a regex, the function will return true
    /// if the regex matches the contents of str.
    ///
    /// ## Examples
    ///
    /// ```
    /// $contains("abracadabra", "bra") => true
    /// $contains("abracadabra", /a.*a/) => true
    /// $contains("abracadabra", /ar.*a/) => false
    /// $contains("Hello World", /wo/) => false
    /// $contains("Hello World", /wo/i) => true
    /// Phone[$contains(number, /^077/)] => { "type": "mobile", "number": "077 7700 1234" }
    pub(crate) fn contains(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Splits the str parameter into an array of substrings. If str is not specified, then the
    /// context value is used as the value of str. It is an error if str is not a string.
    ///
    /// The separator parameter can either be a string or a regular expression (regex).
    /// If it is a string, it specifies the characters within str about which it should be split.
    /// If it is the empty string, str will be split into an array of single characters.
    /// If it is a regex, it splits the string around any sequence of characters that match the regex.
    ///
    /// The optional limit parameter is a number that specifies the maximum number of substrings
    /// to include in the resultant array. Any additional substrings are discarded. If limit is
    /// not specified, then str is fully split with no limit to the size of the resultant array.
    /// It is an error if limit is not a non-negative number.
    ///
    /// ## Examples
    ///
    /// ```
    /// $split("so many words", " ") => [ "so", "many", "words" ]
    /// $split("so many words", " ", 2) => [ "so", "many" ]
    /// $split("too much, punctuation. hard; to read", /[ ,.;]+/) => ["too", "much", "punctuation", "hard", "to", "read"]
    ///
    pub(crate) fn split(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Joins an array of component strings into a single concatenated string with each component
    /// string separated by the optional separator parameter.
    ///
    /// It is an error if the input array contains an item which isn't a string.
    ///
    /// If separator is not specified, then it is assumed to be the empty string, i.e. no separator
    /// between the component strings. It is an error if separator is not a string.
    ///
    /// ## Examples
    ///
    /// ```
    /// $join(['a','b','c']) => "abc"
    /// $split("too much, punctuation. hard; to read", /[ ,.;]+/, 3) ~> $join(', ') => "too, much, punctuation"
    ///
    pub(crate) fn join(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Applies the `str` string to the pattern regular expression and returns an array of objects,
    /// with each object containing information about each occurrence of a match withing `str`.
    ///
    /// The object contains the following fields:
    ///
    /// * match - the substring that was matched by the regex.
    /// * index - the offset (starting at zero) within str of this match.
    /// * groups - if the regex contains capturing groups (parentheses), this contains an array of
    ///            strings representing each captured group.
    ///
    /// If `str` is not specified, then the context value is used as the value of `str`.
    /// It is an error if `str` is not a string.
    ///
    /// ## Examples
    ///
    /// ```
    /// $match("ababbabbcc",/a(b+)/) =>
    /// ```
    /// ```json
    /// [
    ///   {
    ///     "match": "ab",
    ///     "index": 0,
    ///     "groups": ["b"]
    ///   },
    ///   {
    ///     "match": "abb",
    ///     "index": 2,
    ///     "groups": ["bb"]
    ///   },
    ///   {
    ///     "match": "abb",
    ///     "index": 5,
    ///     "groups": ["bb" ]
    ///   }
    /// ]
    /// ```
    pub(crate) fn r#match(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Finds occurrences of pattern within str and replaces them with replacement.
    ///
    /// If str is not specified, then the context value is used as the value of str.
    /// It is an error if str is not a string.
    ///
    /// The pattern parameter can either be a string or a regular expression (regex).
    /// If it is a string, it specifies the substring(s) within str which should be replaced.
    /// If it is a regex, its is used to find .
    ///
    /// The replacement parameter can either be a string or a function. If it is a string,
    /// it specifies the sequence of characters that replace the substring(s) that are matched
    /// by pattern. If pattern is a regex, then the replacement string can refer to the
    /// characters that were matched by the regex as well as any of the captured groups using
    /// a $ followed by a number N:
    ///
    /// * If N = 0, then it is replaced by substring matched by the regex as a whole.
    /// * If N > 0, then it is replaced by the substring captured by the Nth parenthesised group in the regex.
    /// * If N is greater than the number of captured groups, then it is replaced by the empty string.
    /// * A literal $ character must be written as $$ in the replacement string
    ///
    /// If the replacement parameter is a function, then it is invoked for each match occurrence
    /// of the pattern regex. The replacement function must take a single parameter which will be
    /// the object structure of a regex match as described in the $match function; and must
    /// return a string.
    ///
    /// The optional limit parameter, is a number that specifies the maximum number of
    /// replacements to make before stopping. The remainder of the input beyond this limit
    /// will be copied to the output unchanged.
    pub(crate) fn replace(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Parses and evaluates the string expr which contains literal JSON or a JSONata expression
    /// using the current context as the context for evaluation.
    ///
    /// ## Examples
    /// ```
    /// $eval("[1,2,3]") -> [1, 2, 3]
    /// $eval('[1,$string(2),3]') -> [1,"2",3]
    /// ```
    ///
    /// Optionally override the context by specifying the second parameter
    pub(crate) fn eval(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Converts an ASCII string to a base 64 representation. Each each character in the string
    /// is treated as a byte of binary data. This requires that all characters in the string
    /// are in the 0x00 to 0xFF range, which includes all characters in URI encoded strings.
    /// Unicode characters outside of that range are not supported.
    ///
    /// ## Examples
    ///
    /// ```
    /// $base64encode("myuser:mypass") => "bXl1c2VyOm15cGFzcw=="
    /// ```
    pub(crate) fn base64_encode(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Converts base 64 encoded bytes to a string, using a UTF-8 Unicode codepage.
    ///
    /// ## Examples
    ///
    /// ```
    /// $base64decode("bXl1c2VyOm15cGFzcw==") => "myuser:mypass"
    /// ```
    pub(crate) fn base64_decode(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Encodes a Uniform Resource Locator (URL) component by replacing each instance of
    /// certain characters by one, two, three, or four escape sequences representing the
    /// UTF-8 encoding of the character.
    ///
    /// ## Examples
    ///
    /// ```
    /// $encodeUrlComponent("?x=test") => "%3Fx%3Dtest"
    /// ```

    pub(crate) fn encode_url_component(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Encodes a Uniform Resource Locator (URL) by replacing each instance of certain
    /// characters by one, two, three, or four escape sequences representing the UTF-8
    /// encoding of the character.
    ///
    /// ## Examples
    ///
    /// ```
    /// $encodeUrl("https://mozilla.org/?x=шеллы") => "https://mozilla.org/?x=%D1%88%D0%B5%D0%BB%D0%BB%D1%8B"
    /// ```
    pub(crate) fn encode_url(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Decodes a Uniform Resource Locator (URL) component previously created by `encodeUrlComponent`.
    ///
    /// ## Examples
    ///
    /// ```
    /// $decodeUrlComponent("%3Fx%3Dtest") => "?x=test"
    /// ```
    pub(crate) fn decode_url_component(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }

    /// Decodes a Uniform Resource Locator (URL) previously created by `encodeUrl`.
    ///
    /// ## Examples
    /// ```
    /// $decodeUrl("https://mozilla.org/?x=%D1%88%D0%B5%D0%BB%D0%BB%D1%8B") => "https://mozilla.org/?x=шеллы"
    /// ```
    pub(crate) fn decode_url(args: &[Option<JSONataValue>]) -> EvaluationResult {
        todo!()
    }
}

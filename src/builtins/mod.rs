use serde_json::Value;

mod bool;
mod date;
mod object;
mod sequence;

pub(crate) struct BuiltIns;

impl BuiltIns {
    pub(crate) fn evaluate(builtin: BuiltInFunction, args: &[Value]) {
        todo!()
        // TODO: Look into how Boa registers builtins?
    }
}

pub(crate) enum BuiltInFunction {
    // String builtins
    String,
    Length,
    Substring,
    SubstringBefore,
    SubstringAfter,
    Uppercase,
    Lowercase,
    Trim,
    Pad,
    Contains,
    Split,
    Join,
    Match,
    Replace,
    Eval,
    Base64Encode,
    Base64Decode,
    EncodeUrlComponent,
    EncodeUrl,
    DecodeUrlComponent,
    DecodeUrl,

    // Numeric builtins
    Number,
    Abs,
    Floor,
    Ceil,
    Round,
    Power,
    Sqrt,
    Random,
    FormatNumber,
    FormatBase,
    FormatInteger,
    ParseInteger,

    // Boolean builtins
    Boolean,
    Not,
    Exists,

    // Sequence / Array builtins
    Sum,
    Max,
    Min,
    Average,
    Count,
    Append,
    Sort,
    Reverse,
    Shuffle,
    Distinct,
    Zip,

    // Object builtins
    Keys,
    Lookup,
    Spread,
    Merge,
    Each,
    Error,
    Assert,
    Type,

    // Date / Time builtins
    Now,
    Millis,
    FromMillis,
    ToMillis,

    // Higher order
    Map,
    Filter,
    Single,
    Reduce,
    Sift,
}

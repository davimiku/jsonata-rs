use serde_json::Value;

mod bool;
mod date;
mod object;
mod sequence;

pub(crate) struct BuiltIns;

impl BuiltIns {
    // pub(crate) fn evaluate(builtin: BuiltInFunction, args: &[Value]) {
    //     builtin.evaluate(args)
    // }
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

trait EvaluateBuiltIn {
    fn evaluate(&self, args: &[Value]);
}

enum TestEnum {
    // (number[]) -> number
    Max,

    // (string) -> string
    Trim,

    // (array, function(val) -> bool) -> array
    Filter,
}

// impl EvaluateBuiltIn for BuiltInFunction {
//     fn evaluate(&self, args: &[Value]) {
//         match self {
//             // String
//             BuiltInFunction::String => todo!(),
//             BuiltInFunction::Length => todo!(),
//             BuiltInFunction::Substring => todo!(),
//             BuiltInFunction::SubstringBefore => todo!(),
//             BuiltInFunction::SubstringAfter => todo!(),
//             BuiltInFunction::Uppercase => todo!(),
//             BuiltInFunction::Lowercase => todo!(),
//             BuiltInFunction::Trim => todo!(),
//             BuiltInFunction::Pad => todo!(),
//             BuiltInFunction::Contains => todo!(),
//             BuiltInFunction::Split => todo!(),
//             BuiltInFunction::Join => todo!(),
//             BuiltInFunction::Match => todo!(),
//             BuiltInFunction::Replace => todo!(),
//             BuiltInFunction::Eval => todo!(),
//             BuiltInFunction::Base64Encode => todo!(),
//             BuiltInFunction::Base64Decode => todo!(),
//             BuiltInFunction::EncodeUrlComponent => todo!(),
//             BuiltInFunction::EncodeUrl => todo!(),
//             BuiltInFunction::DecodeUrlComponent => todo!(),
//             BuiltInFunction::DecodeUrl => todo!(),

//             // Numeric
//             BuiltInFunction::Number => todo!(),
//             BuiltInFunction::Abs => todo!(),
//             BuiltInFunction::Floor => todo!(),
//             BuiltInFunction::Ceil => todo!(),
//             BuiltInFunction::Round => todo!(),
//             BuiltInFunction::Power => todo!(),
//             BuiltInFunction::Sqrt => todo!(),
//             BuiltInFunction::Random => todo!(),
//             BuiltInFunction::FormatNumber => todo!(),
//             BuiltInFunction::FormatBase => todo!(),
//             BuiltInFunction::FormatInteger => todo!(),
//             BuiltInFunction::ParseInteger => todo!(),

//             // Boolean
//             BuiltInFunction::Boolean => todo!(),
//             BuiltInFunction::Not => todo!(),
//             BuiltInFunction::Exists => todo!(),

//             // Sequence
//             BuiltInFunction::Sum => todo!(),
//             BuiltInFunction::Max => todo!(),
//             BuiltInFunction::Min => todo!(),
//             BuiltInFunction::Average => todo!(),
//             BuiltInFunction::Count => todo!(),
//             BuiltInFunction::Append => todo!(),
//             BuiltInFunction::Sort => todo!(),
//             BuiltInFunction::Reverse => BuiltIns::reverse(args),
//             BuiltInFunction::Shuffle => todo!(),
//             BuiltInFunction::Distinct => todo!(),
//             BuiltInFunction::Zip => todo!(),

//             // Object
//             BuiltInFunction::Keys => todo!(),
//             BuiltInFunction::Lookup => todo!(),
//             BuiltInFunction::Spread => todo!(),
//             BuiltInFunction::Merge => todo!(),
//             BuiltInFunction::Each => todo!(),
//             BuiltInFunction::Error => todo!(),
//             BuiltInFunction::Assert => todo!(),
//             BuiltInFunction::Type => todo!(),

//             // Date / Time
//             BuiltInFunction::Now => todo!(),
//             BuiltInFunction::Millis => todo!(),
//             BuiltInFunction::FromMillis => todo!(),
//             BuiltInFunction::ToMillis => todo!(),

//             // Higher Order
//             BuiltInFunction::Map => todo!(),
//             BuiltInFunction::Filter => todo!(),
//             BuiltInFunction::Single => todo!(),
//             BuiltInFunction::Reduce => todo!(),
//             BuiltInFunction::Sift => todo!(),
//         }
//     }
// }

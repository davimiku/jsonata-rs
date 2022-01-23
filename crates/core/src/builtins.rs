use crate::{
    evaluate::{EvaluationError, EvaluationResult},
    value::{number::JSONataNumber, JSONataValue, JSONataVariables},
};

mod boolean;
mod date;
mod numeric;
mod object;
mod sequence;
mod string;

/// Curries a built-in function to parse the input slice into
/// a single `JSONataValue` argument.
///
/// Invalid number of arguments causes an error to be returned.
///
/// If the expected number of argument are passed but the argument
/// is `None`, it skips the built-in processing and returns an
/// `Ok(None)`. Almost all of the JSONata built-ins functions just
/// propagate the `None` value.
fn one_arg<F>(
    func: F,
    func_name: &'static str,
) -> impl Fn(&[Option<JSONataValue>]) -> EvaluationResult
where
    F: Fn(&JSONataValue) -> EvaluationResult,
{
    move |args: &[Option<JSONataValue>]| match args.len() {
        1 => match args.get(0).unwrap() {
            Some(arg) => func(arg),
            None => Ok(None),
        },
        len => Err(EvaluationError::FunctionIncorrectNumberArguments {
            func_name: func_name.into(),
            expected: 1,
            actual: len,
        }),
    }
}

fn one_arg_no_propogate_none<F>(
    func: F,
    func_name: &'static str,
) -> impl Fn(&[Option<JSONataValue>]) -> EvaluationResult
where
    F: Fn(&Option<JSONataValue>) -> EvaluationResult,
{
    move |args: &[Option<JSONataValue>]| match args.len() {
        1 => func(args.get(0).unwrap()),
        len => Err(EvaluationError::FunctionIncorrectNumberArguments {
            func_name: func_name.to_string(),
            expected: 1,
            actual: len,
        }),
    }
}

fn ensure_string<F>(func: F, func_name: &'static str) -> impl Fn(&JSONataValue) -> EvaluationResult
where
    F: Fn(String) -> EvaluationResult,
{
    move |arg: &JSONataValue| {
        match arg {
            JSONataValue::JSONValue(val) => match &val.0 {
                serde_json::Value::String(s) => {
                    return func(s.clone());
                }
                _ => {}
            },
            _ => {}
        }
        Err(EvaluationError::FunctionInvalidArgument {
            func_name: func_name.to_string(),
            arg_index: 0,
            expected_format: "string".to_string(),
        })
    }
}

fn ensure_number<F>(func: F, func_name: &'static str) -> impl Fn(&JSONataValue) -> EvaluationResult
where
    F: Fn(&JSONataNumber) -> EvaluationResult,
{
    move |arg: &JSONataValue| {
        match arg {
            JSONataValue::JSONValue(val) => match &val.0 {
                serde_json::Value::Number(num) => {
                    let num: JSONataNumber = num.into();
                    return func(&num);
                }
                _ => {}
            },
            _ => {}
        }
        Err(EvaluationError::FunctionInvalidArgument {
            func_name: func_name.to_string(),
            arg_index: 0,
            expected_format: "number".to_string(),
        })
    }
}

/// TODO: Provide a helper function to ensure a value is always a Vec?
/// could be helpful for sequence built-ins
// #[inline]
// fn vecify<F>(func: F) -> impl Fn(&JSONataValue) -> EvaluationResult
// where
//     F: Fn(Vec<JSONataValue>) -> EvaluationResult,
// {
//     // move |arg: &JSONataValue| {
//     //     let vecified: Vec<JSONataValue> = match arg {
//     //         JSONataValue::Value(val) => match val {
//     //             Value::Array(arr) => arr.iter().map(|val| JSONataValue::from(val)).collect(),
//     //             v => todo!(), // vec![v],
//     //         },
//     //         JSONataValue::Function(f) => {
//     //             let g = f.clone();
//     //             vec![JSONataValue::Function(g)]
//     //         }
//     //     };
//     //     func(vecified);
//     //     todo!()
//     // }
// }

pub(crate) struct BuiltIns;

impl BuiltIns {
    pub(crate) fn populate_context(variables: &mut JSONataVariables) {
        // TODO: Add the rest of the built-ins
        BuiltIns::add_builtin(
            variables,
            "count",
            one_arg_no_propogate_none(BuiltIns::count, "count"),
        );

        // boolean
        BuiltIns::add_builtin(variables, "boolean", one_arg(BuiltIns::boolean, "boolean"));
        BuiltIns::add_builtin(variables, "not", one_arg(BuiltIns::not, "not"));
        // BuiltIns::add_builtin(
        //     variables,
        //     "exists",
        //     one_arg_no_propagate_none(BuiltIns::exists),
        // );

        // sequence
        BuiltIns::add_builtin(
            variables,
            "distinct",
            one_arg(BuiltIns::distinct, "distinct"),
        );

        // string
        BuiltIns::add_builtin(variables, "string", one_arg(BuiltIns::string, "string"));
        BuiltIns::add_builtin(
            variables,
            "length",
            one_arg(ensure_string(BuiltIns::length, "length"), "length"),
        );

        // number
        BuiltIns::add_builtin(
            variables,
            "abs",
            one_arg(ensure_number(BuiltIns::abs, "abs"), "abs"),
        );
        BuiltIns::add_builtin(
            variables,
            "floor",
            one_arg(ensure_number(BuiltIns::floor, "floor"), "floor"),
        );
    }

    /// Adds the built-in function to a variables hashmap, which is generally available
    /// to the currently running program.
    fn add_builtin<N, B>(variables: &mut JSONataVariables, ident: N, builtin: B)
    where
        N: Into<String> + Clone,
        B: 'static + Fn(&[Option<JSONataValue>]) -> EvaluationResult,
    {
        let func = JSONataValue::from_builtin(builtin, ident.clone());
        variables.insert(ident.into(), func);
    }
}

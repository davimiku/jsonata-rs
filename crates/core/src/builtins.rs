use crate::{
    evaluate::{EvaluationError, EvaluationResult},
    value::{JSONataValue, JSONataVariables},
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
#[inline]
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
        // Add number of required arguments here?
        BuiltIns::add_builtin(variables, "count", BuiltIns::count);

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
        BuiltIns::add_builtin(variables, "string", one_arg(BuiltIns::string, "string"))
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

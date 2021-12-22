use crate::{
    evaluate::{EvaluationError, EvaluationResult, JSONataVariables},
    value::JSONataValue,
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
/// If the expected number of argument are passed but the argument
/// is `None`, it skips the built-in processing and returns an
/// `Ok(None)` as almost all of the JSONata built-ins just propagate
/// the `None` value.
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
        len => Err(EvaluationError::function_incorrect_num_arguments(
            func_name, 1, len,
        )),
    }
}

/// TODO: finish documenting
///
/// produces a new function...
/// similar to one_arg but does not propagate the None, and is used for functions
/// that specifically need to know if the arg is None or not
/// FIXME: How can we get the function name in here without being annoying?
#[inline]
fn one_arg_no_propagate_none<F>(func: F) -> impl Fn(&[Option<JSONataValue>]) -> EvaluationResult
where
    F: Fn(&Option<JSONataValue>) -> EvaluationResult,
{
    move |args: &[Option<JSONataValue>]| match args.get(0) {
        Some(arg) => func(arg),
        None => Err(EvaluationError::function_incorrect_num_arguments(
            "FIXME", 1, 0,
        )),
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
        BuiltIns::add_builtin(
            variables,
            "exists",
            one_arg_no_propagate_none(BuiltIns::exists),
        );

        // sequence
        BuiltIns::add_builtin(
            variables,
            "distinct",
            one_arg(BuiltIns::distinct, "distinct"),
        );
    }

    /// Adds the built-in function to a variables hashmap, which is generally available
    /// to the currently running program.
    ///
    /// FIXME: 'static lifetime may be wrong here.
    fn add_builtin<N, B: 'static>(variables: &mut JSONataVariables, ident: N, builtin: B)
    where
        N: Into<String> + Clone,
        B: Fn(&[Option<JSONataValue>]) -> EvaluationResult,
    {
        let func = JSONataValue::from_func(builtin, ident.clone());
        variables.insert(ident.into(), Some(func).into());
    }
}

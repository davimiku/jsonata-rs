use crate::{
    ast::expr::Expression,
    evaluate::{Context, EvaluationResult},
};

#[derive(Debug, PartialEq)]
pub(crate) struct ConcatExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl ConcatExpression {
    pub(crate) fn evaluate(&self, context: &mut Context) -> EvaluationResult {
        let left = self.lhs.evaluate(context)?;
        let right = self.rhs.evaluate(context)?;
        dbg!(format!("{:?}", &left));
        dbg!(format!("{:?}", &right));
        Ok(Some(
            match (left, right) {
                (None, None) => "".into(),
                (Some(a), None) => format!("{}", a),
                (None, Some(b)) => format!("{}", b),
                (Some(a), Some(b)) => format!("{}{}", a, b),
            }
            .into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::literal::LiteralExpression;

    use super::*;

    #[test]
    fn concat_empty() {
        let expr = ConcatExpression {
            lhs: Box::new(LiteralExpression::from("").into()),
            rhs: Box::new(LiteralExpression::from("").into()),
        };
        let actual = expr.evaluate(&mut Context::default()).unwrap().unwrap();

        assert_eq!(actual, "".into());
    }
}

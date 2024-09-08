use crate::errored;
use crate::query::structs::expression::ExpressionResult::{Bool, Int, Str};
use crate::query::structs::expression::{ExpressionOperator, ExpressionResult};
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Syntax;

pub struct ExpressionComparator;

impl ExpressionComparator {
    pub fn compare_ints(
        l: i64,
        r: i64,
        op: &ExpressionOperator,
    ) -> Result<ExpressionResult, Errored> {
        match op {
            ExpressionOperator::Equals => Ok(Bool(l == r)),
            ExpressionOperator::NotEquals => Ok(Bool(l != r)),
            ExpressionOperator::GreaterThan => Ok(Bool(l > r)),
            ExpressionOperator::LessThan => Ok(Bool(l < r)),
            ExpressionOperator::GreaterOrEqual => Ok(Bool(l >= r)),
            ExpressionOperator::LessOrEqual => Ok(Bool(l <= r)),
            _ => errored!(Syntax, "invalid comparison for ints: {:?}", op),
        }
    }

    pub fn compare_str(
        l: &str,
        r: &str,
        op: &ExpressionOperator,
    ) -> Result<ExpressionResult, Errored> {
        match op {
            ExpressionOperator::Equals => Ok(Bool(l == r)),
            ExpressionOperator::NotEquals => Ok(Bool(l != r)),
            ExpressionOperator::GreaterThan => Ok(Bool(l > r)),
            ExpressionOperator::LessThan => Ok(Bool(l < r)),
            ExpressionOperator::GreaterOrEqual => Ok(Bool(l >= r)),
            ExpressionOperator::LessOrEqual => Ok(Bool(l <= r)),
            _ => errored!(Syntax, "invalid comparison for str: {:?}", op),
        }
    }

    pub fn compare_bools(
        l: bool,
        r: bool,
        op: &ExpressionOperator,
    ) -> Result<ExpressionResult, Errored> {
        match op {
            ExpressionOperator::And => Ok(Bool(l && r)),
            ExpressionOperator::Or => Ok(Bool(l || r)),
            ExpressionOperator::Not => Ok(Bool(!l)),
            _ => errored!(Syntax, "invalid comparison for bool: {:?}", op),
        }
    }
}

impl ExpressionResult {
    pub fn compare(&self, other: &ExpressionResult) -> Result<std::cmp::Ordering, Errored> {
        match (self, other) {
            (Int(a), Int(b)) => Ok(a.cmp(b)),
            (Str(a), Str(b)) => Ok(a.cmp(b)),
            (Bool(a), Bool(b)) => Ok(a.cmp(b)),
            _ => errored!(
                Syntax,
                "Cannot compare different types: {:?} and {:?}",
                self,
                other
            ),
        }
    }
}

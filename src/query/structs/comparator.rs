use crate::errored;
use crate::query::structs::expression::ExpressionResult::{Bool, Int, Str};
use crate::query::structs::expression::{ExpressionOperator, ExpressionResult};
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Syntax;

/// Comparador de expresiones que permite comparar diferentes tipos de datos.
/// Es utilizado principalmente con los valores "hoja" de las expresiones SQL.
pub struct ExpressionComparator;

impl ExpressionComparator {
    /// Compara dos enteros utilizando el operador especificado.
    ///
    /// # Parámetros
    /// - `l`: El primer entero a comparar.
    /// - `r`: El segundo entero a comparar.
    /// - `op`: El operador de comparación a utilizar.
    ///
    /// # Retorno
    /// Retorna un `Result` que contiene un `ExpressionResult` con el resultado de la comparación,
    /// o un error `Errored` si el operador no es válido para enteros.
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

    /// Compara dos cadenas de texto utilizando el operador especificado.
    ///
    /// # Parámetros
    /// - `l`: La primera cadena de texto a comparar.
    /// - `r`: La segunda cadena de texto a comparar.
    /// - `op`: El operador de comparación a utilizar.
    ///
    /// # Retorno
    /// Retorna un `Result` que contiene un `ExpressionResult` con el resultado de la comparación,
    /// o un error `Errored` si el operador no es válido para cadenas de texto.
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

    /// Compara dos valores booleanos utilizando el operador especificado.
    ///
    /// # Parámetros
    /// - `l`: El primer valor booleano a comparar.
    /// - `r`: El segundo valor booleano a comparar.
    /// - `op`: El operador de comparación a utilizar.
    ///
    /// # Retorno
    /// Retorna un `Result` que contiene un `ExpressionResult` con el resultado de la comparación,
    /// o un error `Errored` si el operador no es válido para valores booleanos.
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

    /// Compara dos resultados de expresiones para determinar su orden relativo.
    ///
    /// # Parámetros
    /// - `this`: El primer resultado de expresión a comparar.
    /// - `other`: El segundo resultado de expresión a comparar.
    ///
    /// # Retorno
    /// Retorna un `Result` que contiene un valor `std::cmp::Ordering` que indica el orden relativo
    /// de los dos resultados, o un error `Errored` si los tipos de los resultados no son comparables.
    pub fn compare_ordering(
        this: &ExpressionResult,
        other: &ExpressionResult,
    ) -> Result<std::cmp::Ordering, Errored> {
        match (this, other) {
            (Int(a), Int(b)) => Ok(a.cmp(b)),
            (Str(a), Str(b)) => Ok(a.cmp(b)),
            (Bool(a), Bool(b)) => Ok(a.cmp(b)),
            _ => errored!(
                Syntax,
                "Cannot compare different types: {:?} and {:?}",
                this,
                other
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::structs::expression::ExpressionOperator::*;
    use std::cmp::Ordering::*;

    #[test]
    fn test_compare_ints() {
        assert_eq!(
            ExpressionComparator::compare_ints(5, 5, &Equals).unwrap(),
            Bool(true)
        );
        assert_eq!(
            ExpressionComparator::compare_ints(5, 4, &NotEquals).unwrap(),
            Bool(true)
        );
        assert_eq!(
            ExpressionComparator::compare_ints(5, 4, &GreaterThan).unwrap(),
            Bool(true)
        );
        assert_eq!(
            ExpressionComparator::compare_ints(4, 5, &LessThan).unwrap(),
            Bool(true)
        );
        assert_eq!(
            ExpressionComparator::compare_ints(5, 5, &GreaterOrEqual).unwrap(),
            Bool(true)
        );
        assert_eq!(
            ExpressionComparator::compare_ints(4, 5, &LessOrEqual).unwrap(),
            Bool(true)
        );
    }

    #[test]
    fn test_compare_ints_invalid() {
        assert!(ExpressionComparator::compare_ints(5, 5, &And).is_err());
    }

    #[test]
    fn test_compare_str() {
        assert_eq!(
            ExpressionComparator::compare_str("a", "a", &Equals).unwrap(),
            Bool(true)
        );
        assert_eq!(
            ExpressionComparator::compare_str("a", "b", &NotEquals).unwrap(),
            Bool(true)
        );
        assert_eq!(
            ExpressionComparator::compare_str("b", "a", &GreaterThan).unwrap(),
            Bool(true)
        );
        assert_eq!(
            ExpressionComparator::compare_str("a", "b", &LessThan).unwrap(),
            Bool(true)
        );
        assert_eq!(
            ExpressionComparator::compare_str("a", "a", &GreaterOrEqual).unwrap(),
            Bool(true)
        );
        assert_eq!(
            ExpressionComparator::compare_str("a", "b", &LessOrEqual).unwrap(),
            Bool(true)
        );
    }

    #[test]
    fn test_compare_str_invalid() {
        assert!(ExpressionComparator::compare_str("a", "a", &And).is_err());
    }

    #[test]
    fn test_compare_bools() {
        assert_eq!(
            ExpressionComparator::compare_bools(true, false, &And).unwrap(),
            Bool(false)
        );
        assert_eq!(
            ExpressionComparator::compare_bools(true, false, &Or).unwrap(),
            Bool(true)
        );
        assert_eq!(
            ExpressionComparator::compare_bools(true, false, &Not).unwrap(),
            Bool(false)
        );
    }

    #[test]
    fn test_compare_bools_invalid() {
        assert!(ExpressionComparator::compare_bools(true, false, &Equals).is_err());
    }

    #[test]
    fn test_compare_ordering_ints() {
        assert_eq!(
            ExpressionComparator::compare_ordering(&Int(5), &Int(5)).unwrap(),
            Equal
        );
        assert_eq!(
            ExpressionComparator::compare_ordering(&Int(5), &Int(4)).unwrap(),
            Greater
        );
        assert_eq!(
            ExpressionComparator::compare_ordering(&Int(4), &Int(5)).unwrap(),
            Less
        );
    }

    #[test]
    fn test_compare_ordering_strs() {
        assert_eq!(
            ExpressionComparator::compare_ordering(&Str("a".to_string()), &Str("a".to_string()))
                .unwrap(),
            Equal
        );
        assert_eq!(
            ExpressionComparator::compare_ordering(&Str("b".to_string()), &Str("a".to_string()))
                .unwrap(),
            Greater
        );
        assert_eq!(
            ExpressionComparator::compare_ordering(&Str("a".to_string()), &Str("b".to_string()))
                .unwrap(),
            Less
        );
    }

    #[test]
    fn test_compare_ordering_invalid() {
        assert!(ExpressionComparator::compare_ordering(&Int(5), &Str("a".to_string())).is_err());
    }
}

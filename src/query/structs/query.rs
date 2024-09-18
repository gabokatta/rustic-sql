use crate::errored;
use crate::query::builder::delete::DeleteBuilder;
use crate::query::builder::insert::InsertBuilder;
use crate::query::builder::select::SelectBuilder;
use crate::query::builder::update::UpdateBuilder;
use crate::query::builder::{get_kind, Builder};
use crate::query::structs::expression::ExpressionNode;
use crate::query::structs::operation::Operation;
use crate::query::structs::operation::Operation::{Delete, Insert, Select, Unknown, Update};
use crate::query::structs::ordering::Ordering;
use crate::query::structs::token::Token;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Syntax;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};

/// Estructura que representa una consulta dentro de RusticSQL.
///
/// La consulta incluye la operación a realizar, la tabla, las columnas, los valores para insertar,
/// las actualizaciones, las condiciones y el ordenamiento, algunos de estos campos pueden quedar
/// con valores default en caso de no aplicar.
pub struct Query {
    /// La operación que se debe realizar.
    pub operation: Operation,
    /// La tabla sobre la que se realiza la operación.
    pub table: String,
    /// Las columnas involucradas en la consulta.
    pub columns: Vec<Token>,
    /// Los valores a insertar en caso de una operación de inserción.
    pub inserts: Vec<Vec<Token>>,
    /// Las actualizaciones a realizar en caso de una operación de actualización.
    pub updates: Vec<ExpressionNode>,
    /// Las condiciones para filtrar los resultados de la consulta.
    pub conditions: ExpressionNode,
    /// El criterio de ordenamiento para los resultados.
    pub ordering: Vec<Ordering>,
}

impl Query {
    /// Crea una nueva consulta a partir de una lista de tokens.
    ///
    /// La función intenta identificar el tipo de operación
    /// y construir la consulta correspondiente usando el builder adecuado.
    ///
    /// # Parámetros
    ///
    /// * `tokens` - La lista de tokens obtenida de tokenizar un string que representaba la consulta.
    ///
    /// # Retorna
    ///
    /// Un `Result` que contiene la consulta construida o un error en caso de que la consulta no sea
    /// válida.
    pub fn from(tokens: Vec<Token>) -> Result<Self, Errored> {
        let mut tokens = VecDeque::from(tokens);
        let kind = get_kind(tokens.pop_front());
        match kind {
            Unknown => errored!(Syntax, "la consulta no comienza con una operación válida."),
            Select => SelectBuilder::new(tokens).build(),
            Update => UpdateBuilder::new(tokens).build(),
            Delete => DeleteBuilder::new(tokens).build(),
            Insert => InsertBuilder::new(tokens).build(),
        }
    }
}

impl Default for Query {
    /// Devuelve un valor default para `Query`.
    fn default() -> Self {
        Self {
            operation: Unknown,
            table: "".to_string(),
            columns: vec![],
            inserts: vec![],
            updates: vec![],
            conditions: ExpressionNode::default(),
            ordering: vec![],
        }
    }
}

impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let fields: Vec<&str> = self.columns.iter().map(|f| f.value.as_str()).collect();
        writeln!(f, "Tipo de Consulta: [{:?}]", self.operation)?;
        writeln!(f, "Tabla: {:?}", self.table)?;
        writeln!(f, "Columnas: {:?}", fields)?;
        writeln!(f, "Inserts {{ ")?;
        for insert in &self.inserts {
            let values: Vec<&String> = insert.iter().map(|t| &t.value).collect();
            writeln!(f, "   {:?}", values)?;
        }
        writeln!(f, "}} ")?;
        writeln!(f, "Actualizaciones {{ ")?;
        for up in &self.updates {
            if let Ok((l, r)) = up.as_leaf_tuple() {
                writeln!(f, "   {} -> {}", l.value, r.value)?;
            }
        }
        writeln!(f, "}} ")?;
        writeln!(f, "Actualizaciones: {:?}", self.updates)?;
        writeln!(f, "Condiciones: {:?}", self.conditions)?;
        writeln!(f, "Ordenamiento: {:?}", self.ordering)
    }
}

impl Debug for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self)
    }
}

#[cfg(test)]
mod test {
    use crate::query::structs::query::Query;
    use crate::query::structs::token::Token;
    use crate::utils::errors::Errored;

    #[test]
    fn test_invalid_query() {
        let tokens = vec![Token::default()];
        let result = Query::from(tokens);
        assert!(
            result.is_err(),
            "debería dar error con un token desconocido."
        );
        match result {
            Err(Errored::Syntax(msg)) => assert!(msg.contains("operación válida")),
            _ => panic!("se esperaba un error de sintaxis para el primer token de la consulta."),
        }
    }
}

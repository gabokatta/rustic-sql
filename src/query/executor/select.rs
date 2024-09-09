use crate::errored;
use crate::query::executor::Executor;
use crate::query::structs::comparator::ExpressionComparator;
use crate::query::structs::expression::ExpressionNode;
use crate::query::structs::ordering::OrderKind;
use crate::query::structs::row::Row;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Column;
use crate::utils::files::{extract_header, get_table_file, split_csv};
use std::cmp::Ordering;
use std::io::{BufRead, BufReader};

impl Executor {
    /// Ejecuta la operación de selección de registros en la tabla especificada.
    /// # Proceso
    ///
    /// 1. Abre el archivo de la tabla especificada.
    /// 2. Lee el encabezado del archivo para obtener los nombres de las columnas.
    /// 3. Valida las columnas de proyección especificadas en la consulta SQL.
    /// 4. Lee y procesa cada línea del archivo:
    ///    - Divide la línea en campos y los convierte en una fila (`Row`).
    ///    - Verifica si la fila cumple con las condiciones de la consulta.
    /// 5. Ordena las filas coincidentes según los criterios de ordenamiento.
    /// 6. Imprime el encabezado y las filas coincidentes en la salida estándar.
    ///
    /// # Errores
    ///
    /// Puede retornar un error si ocurre un problema al abrir el archivo de la tabla, leer el encabezado,
    /// procesar las líneas, validar las columnas de proyección o realizar el ordenamiento.
    pub fn run_select(&mut self) -> Result<(), Errored> {
        let table = get_table_file(&self.table_path)?;
        let mut reader = BufReader::new(&table);
        let header = extract_header(&mut reader)?;
        self.validate_projection(&header)?;
        let mut matched_rows: Vec<Row> = vec![];
        for line in reader.lines() {
            let l = line?;
            let fields = split_csv(&l);
            let mut row = Row::new(&header);
            row.read_new_row(fields)?;
            if row.matches_condition(&self.query)? {
                matched_rows.push(row)
            }
        }
        self.sort_rows(&mut matched_rows, &header)?;
        self.output_projection(&header, &matched_rows);
        Ok(())
    }

    /// Ordena las filas coincidentes según los criterios de ordenamiento especificados en la consulta.
    ///
    /// Este método toma las filas coincidentes y las ordena en función de los campos y el tipo de ordenamiento
    /// especificados en `self.query.ordering`. Verifica si los campos de ordenamiento existen en el encabezado
    /// de la tabla y realiza la comparación correspondiente.
    ///
    /// Si hay varios ordenamientos en la consulta, primero se evalua uno y si el resultado es igual,
    /// se compara por el siguiente.
    ///
    /// # Errores
    ///
    /// Retorna un error si alguno de los campos de ordenamiento no existe en el encabezado.
    ///
    /// # Ejemplo
    ///
    /// Este método es llamado internamente por `run_select`, por lo que no tiene un ejemplo de uso independiente.
    fn sort_rows(&mut self, matched_rows: &mut [Row], header: &[String]) -> Result<(), Errored> {
        for order in &self.query.ordering {
            if !header.contains(&order.field.value) {
                errored!(
                    Column,
                    "order by failed, column {} does not exist",
                    &order.field.value
                )
            }
        }
        matched_rows.sort_by(|a, b| {
            for order in &self.query.ordering {
                let l = ExpressionNode::get_variable_value(&a.values, &order.field);
                let r = ExpressionNode::get_variable_value(&b.values, &order.field);
                if let (Ok(a), Ok(b)) = (l, r) {
                    let comparison_result = match order.kind {
                        OrderKind::Asc => ExpressionComparator::compare_ordering(&a, &b)
                            .unwrap_or(Ordering::Equal),
                        OrderKind::Desc => ExpressionComparator::compare_ordering(&b, &a)
                            .unwrap_or(Ordering::Equal),
                    };
                    if comparison_result != Ordering::Equal {
                        return comparison_result;
                    }
                }
            }
            Ordering::Equal
        });
        Ok(())
    }

    /// Imprime las filas coincidentes en la salida estándar.
    ///
    /// Este método toma las filas coincidentes y las imprime en la salida estándar, proyectando solo las columnas
    /// especificadas en la consulta SQL (`self.query.columns`).
    ///
    /// Ademas, se encarga de imprimir la proyección del header del csv.
    /// Si las columnas proyectadas son vacias, se asume que el operador * esta siendo usado,
    /// de lo contrario se imprime el header proyectado a las columnas.
    ///
    /// # Ejemplo
    ///
    /// Este método es llamado internamente por `run_select`, por lo que no tiene un ejemplo de uso independiente.
    fn output_projection(&self, header: &[String], matched_rows: &[Row]) {
        let mut columns = vec![];
        if self.query.columns.is_empty() {
            println!("{}", header.join(","));
        } else {
            columns = self
                .query
                .columns
                .iter()
                .map(|t| t.value.to_string())
                .collect();
            println!("{}", columns.join(","));
        }
        for row in matched_rows {
            row.print_projection(&columns)
        }
    }

    /// Valida que todas las columnas especificadas en la proyección existan en el encabezado de la tabla.
    ///
    /// Este método verifica que todas las columnas que se desean proyectar en la consulta SQL (`self.query.columns`)
    /// estén presentes en el encabezado del archivo de la tabla. Si alguna columna no existe, retorna un error.
    ///
    /// # Errores
    ///
    /// Retorna un error si alguna columna en la proyección no existe en el encabezado.
    ///
    /// # Ejemplo
    ///
    /// Este método es llamado internamente por `run_select`, por lo que no tiene un ejemplo de uso independiente.
    fn validate_projection(&self, header: &[String]) -> Result<(), Errored> {
        for column in &self.query.columns {
            let value = &column.value;
            if !header.contains(value) {
                errored!(
                    Column,
                    "column {} in projection does not exist in table.",
                    value
                )
            }
        }
        Ok(())
    }
}

use crate::query::executor::Executor;
use crate::query::structs::row::Row;
use crate::utils::errors::Errored;
use crate::utils::files;
use crate::utils::files::{extract_header, get_table_file};
use std::io::{BufReader, Write};

impl Executor {
    /// Ejecuta la operación de inserción de registros en la tabla especificada.
    /// # Proceso
    ///
    /// 1. Abre el archivo de la tabla especificada en `self.table_path`.
    /// 2. Lee el encabezado del archivo para obtener los nombres de las columnas.
    /// 3. Asegura que el archivo termine en una nueva línea.
    /// 4. Para cada inserción en `self.query.inserts`:
    ///    - Convierte los valores de la inserción en una fila de valores.
    ///    - Crea una nueva fila (`Row`) y la llena con los valores.
    ///    - Escribe la fila como una línea CSV en el archivo.
    ///
    /// # Errores
    ///
    /// Puede retornar un error si ocurre un problema al abrir el archivo de la tabla, leer el encabezado,
    /// agregar la nueva línea o escribir en el archivo.
    pub fn run_insert(&self) -> Result<(), Errored> {
        let mut table = get_table_file(&self.table_path)?;
        let mut reader = BufReader::new(&table);
        let header = extract_header(&mut reader)?;
        files::make_file_end_in_newline(&mut table)?;
        for insert in &self.query.inserts {
            let fields: Vec<String> = insert.iter().map(|t| t.value.to_string()).collect();
            let mut row = Row::new(&header);
            row.clear()?;
            row.insert_values(&self.query.columns, fields)?;
            writeln!(table, "{}", row.as_csv_row())?
        }
        Ok(())
    }
}

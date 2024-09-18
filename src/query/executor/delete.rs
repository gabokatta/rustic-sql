use crate::query::executor::Executor;
use crate::query::structs::row::Row;
use crate::utils::errors::Errored;
use crate::utils::files::{
    delete_temp_file, extract_header, get_table_file, get_temp_file, split_csv,
};
use std::io::{BufRead, BufReader, BufWriter, Write};

impl Executor {
    /// Ejecuta la operación de eliminación de registros en la tabla especificada.
    ///
    /// # Proceso
    ///
    /// 1. Abre el archivo de la tabla especificada y crea un archivo temporal para escribir los registros que no serán eliminados.
    /// 2. Lee el encabezado del archivo original y lo escribe en el archivo temporal.
    /// 3. Procesa cada línea del archivo original:
    ///    - Divide la línea en campos y los convierte en una fila (`Row`).
    ///    - Verifica si la fila cumple con las condiciones de eliminación.
    ///    - Si la fila coincide con la condición de eliminación, la omite y no la escribe en el archivo temporal.
    ///    - Si no coincide con la condición de eliminación, escribe la línea original en el archivo temporal.
    /// 4. Una vez procesadas todas las líneas, elimina el archivo original y renombra el archivo temporal para reemplazar el archivo original.
    ///
    /// # Errores
    ///
    /// Puede retornar un error si ocurre un problema al abrir los archivos, leer el encabezado, procesar las líneas o eliminar el archivo temporal.
    pub fn run_delete(&self) -> Result<(), Errored> {
        let table = get_table_file(&self.table_path)?;
        let (temp_table, temp_path) = get_temp_file(&self.query.table, &self.table_path)?;
        let mut reader = BufReader::new(&table);
        let mut writer = BufWriter::new(temp_table);
        let header = extract_header(&mut reader)?;
        writeln!(writer, "{}", header.join(","))?;
        for line in reader.lines() {
            let l = line?;
            let fields = split_csv(&l);
            let mut row = Row::new(&header);
            row.read_new_row(fields)?;
            if row.matches_condition(&self.query)? {
                continue;
            } else {
                writeln!(writer, "{}", l)?
            }
        }
        delete_temp_file(&self.table_path, &temp_path)?;
        Ok(())
    }
}

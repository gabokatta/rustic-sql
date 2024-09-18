use crate::errored;
use crate::utils::errors::Errored;
use crate::utils::errors::Errored::Default;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::{fs, thread};

const TEMP_EXTENSION: &str = "tmp";
const CSV_EXTENSION: &str = "csv";
const CSV_SEPARATOR: &str = ",";

/// Extrae el encabezado de un archivo CSV.
///
/// # Parámetros
///
/// - `reader`: Un `BufReader` que envuelve un `File` desde el cual leer el encabezado.
///
/// # Retorna
///
/// Devuelve un `Result` que contiene un `Vec<String>` con los nombres de las columnas si tiene éxito, o un `Errored` en caso de error.
pub fn extract_header(reader: &mut BufReader<&File>) -> Result<Vec<String>, Errored> {
    let mut header = String::new();
    reader.read_line(&mut header)?;
    Ok(split_csv(&header))
}

/// Divide una línea CSV en un vector de strings.
///
/// # Parámetros
///
/// - `line`: La línea CSV que se desea dividir.
///
/// # Retorna
///
/// Devuelve un `Vec<String>` con los valores separados.
///
/// # Ejemplo
///
/// ```rust
/// use rustic_sql::utils::files::split_csv;
/// let line = "id, id_cliente ,      email ";
/// let result = split_csv(line);
/// println!("{:?}", result); // Imprime ["id", "id_cliente", "email"]
/// ```
pub fn split_csv(line: &str) -> Vec<String> {
    line.split(CSV_SEPARATOR)
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>()
}

/// Obtiene la ruta completa del archivo CSV para una tabla dada.
///
/// # Parámetros
///
/// - `dir_path`: El directorio donde buscar.
/// - `table_name`: El nombre de la tabla.
///
/// # Retorna
///
/// Devuelve un `Result` que contiene un `PathBuf` con la ruta al archivo de la tabla si tiene éxito, o un `Errored` en caso de error.
pub fn get_table_path(dir_path: &Path, table_name: &str) -> Result<PathBuf, Errored> {
    let table_path = dir_path.join(table_name).with_extension(CSV_EXTENSION);
    if !table_path.is_file() {
        errored!(
            Default,
            "table {} does not exist in directory: {}",
            table_name,
            dir_path.display()
        );
    }
    Ok(table_path)
}

/// Genera un identificador único para un archivo temporal.
///
/// # Retorna
///
/// Devuelve un `u64` que representa el identificador único.
///
/// # Ejemplo
///
/// ```rust
/// use rustic_sql::utils::files::get_temp_id;
/// let id = get_temp_id();
/// println!("{}", id);
/// ```
pub fn get_temp_id() -> u64 {
    let id = thread::current().id();
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    hasher.finish()
}

/// Crea un archivo temporal para una tabla dada.
///
/// # Parámetros
///
/// - `table_name`: El nombre de la tabla.
/// - `table_path`: La ruta del directorio en donde debe estar contenida la tabla.
///
/// # Retorna
///
/// Devuelve un `Result` que contiene una tupla con un `File` y un `PathBuf` con la ruta al archivo temporal, o un `Errored` en caso de error.
pub fn get_temp_file(table_name: &str, table_path: &Path) -> Result<(File, PathBuf), Errored> {
    let id = get_temp_id();
    let table_path = table_path
        .with_file_name(format!("{}_{}", table_name, id))
        .with_extension(TEMP_EXTENSION);
    Ok((
        File::options()
            .create_new(true)
            .read(true)
            .write(true)
            .truncate(true)
            .open(&table_path)?,
        table_path,
    ))
}

/// Elimina un archivo temporal, renombrándolo a la ruta de la tabla.
///
/// # Parámetros
///
/// - `table_path`: La ruta al archivo de la tabla.
/// - `temp_path`: La ruta al archivo temporal.
///
/// # Retorna
///
/// Devuelve un `Result` que indica si la operación tuvo éxito o un `Errored` en caso de error.
///
/// # Ejemplo
///
/// ```rust
/// use std::path::Path;
/// use rustic_sql::utils::files::delete_temp_file;
///
/// let table_path = Path::new("tests/unit_tables/clientes.csv");
/// let temp_path = Path::new("tests/unit_tables/clientes.csv.tmp");
/// //delete_temp_file(table_path, temp_path)?;
/// ```
pub fn delete_temp_file(table_path: &Path, temp_path: &Path) -> Result<(), Errored> {
    if let Some(ex) = temp_path.extension() {
        if ex.to_string_lossy() != TEMP_EXTENSION {
            errored!(Default, "tried to delete non_temporary file.")
        }
    }
    fs::rename(temp_path, table_path)?;
    Ok(())
}

/// Asegura que un archivo termine en una nueva línea.
///
/// # Parámetros
///
/// - `file`: El archivo que se desea verificar.
///
/// # Retorna
///
/// Devuelve un `Result` que indica si la operación tuvo éxito o un `Errored` en caso de error.
///
pub fn make_file_end_in_newline(file: &mut File) -> Result<(), Errored> {
    file.seek(SeekFrom::End(0))?;
    if file.metadata()?.len() == 0 {
        return Ok(());
    }
    let mut last_byte = [0; 1];
    file.seek(SeekFrom::End(-1))?;
    file.read_exact(&mut last_byte)?;
    if last_byte[0] != b'\n' {
        file.write_all(b"\n")?;
    }
    Ok(())
}

/// Obtiene un archivo en modo lectura y adición.
///
/// # Parámetros
///
/// - `table_path`: La ruta al archivo de la tabla.
///
/// # Retorna
///
/// Devuelve un `Result` que contiene un `File` en modo lectura y adición, o un `Errored` en caso de error.
///
pub fn get_table_file(table_path: &Path) -> Result<File, Errored> {
    Ok(File::options().read(true).append(true).open(table_path)?)
}

/// Valida si una ruta es un directorio existente y no vacío.
///
/// # Parámetros
///
/// - `dir`: La ruta al directorio.
///
/// # Retorna
///
/// Devuelve un `Result` que contiene un `&Path` si la validación es exitosa, o un `Errored` en caso de error.
///
pub fn validate_path(dir: &str) -> Result<&Path, Errored> {
    let path = Path::new(dir);
    if !path.exists() {
        errored!(Default, "path '{dir}' does not exist");
    } else if !path.is_dir() {
        errored!(Default, "path '{dir}' is not a valid directory");
    } else if path.read_dir()?.next().is_none() {
        errored!(Default, "path '{dir}' is an empty directory");
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_header() {
        let file = File::open("tests/unit_tables/ordenes.csv").unwrap();
        let mut reader = BufReader::new(&file);
        let header = extract_header(&mut reader).unwrap();
        assert_eq!(header, vec!["id", "id_cliente", "producto", "cantidad"]);
    }

    #[test]
    fn test_split_csv() {
        let line = "id, id_cliente ,      email ";
        let result = split_csv(line);
        assert_eq!(result, vec!["id", "id_cliente", "email"]);
    }

    #[test]
    fn test_get_bad_table_path() {
        let dir = Path::new("/dir/sin_unit_tables");
        let table_name = "ordenes";
        let result = get_table_path(dir, table_name);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_temp_file() {
        let table_name = "ordenes";
        let table_path = Path::new("tests/tables");
        let result = get_temp_file(table_name, table_path);
        assert!(result.is_ok());
        let (_, temp_path) = result.unwrap();
        fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn test_delete_temp_file() {
        let table_path = Path::new("tests/unit_tables/ordenes.csv");
        let (_, t_path) = get_temp_file("ordenes", table_path).unwrap();
        fs::copy(table_path, &t_path).unwrap();
        let result = delete_temp_file(table_path, &t_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_non_temporary_file_error() {
        let table_path = Path::new("tests/unit_tables/ordenes.csv");
        let temp_path = Path::new("tests/unit_tables/clientes.csv");
        let result = delete_temp_file(table_path, temp_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_bad_table_file() {
        let table_path = Path::new("/dir/unit_tables/no_existo.csv");
        let result = get_table_file(table_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_path_success() {
        let path = Path::new("tests/unit_tables");
        let result = validate_path(path.to_str().unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_path_not_exist() {
        let result = validate_path("/dir/sin_unit_tables/");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_path_not_a_directory() {
        let result = validate_path("/test/tables/clientes.csv");
        assert!(result.is_err());
    }
}

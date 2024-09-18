# rustic-sql 🦀 ![workflow TP](https://github.com/gabokatta/rustic-sql/actions/workflows/rust.yml/badge.svg)

> [!IMPORTANT]
> ¿Como correr la app?
> - Para las queries de tipo SELECT puedes redirigir la salida de STDOUT a un archivo o simplemente ver la salida por pantalla.
> ```BASH
>cargo run -- ruta/a/tablas "SELECT * FROM table" > output.csv
>```
> - Para las otras queries, simplemente se aplican los cambios sobre los archivos.
> ```BASH
>cargo run -- ruta/a/tablas "INSERT INTO table (id, name) VALUES (1, 'gabriel');"
>```
___
> [!TIP]
> ¿Como testear la app?
>```BASH
>cargo test --all
>```
___
> [!NOTE]
> ¿Como levantar la documentación?
>```BASH
>cargo doc --open
>```
> Alternativamente se puede visualizar por [github!](https://gabokatta.github.io/rustic-sql)

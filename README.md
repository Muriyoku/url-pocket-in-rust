# Description 

A simple, Rust based terminal application for save your urls in "pockets" (files`.txt`).

The project is merely for study purposes. 

Before to follow this guie, make sure to have `Rust` installed in your machine then clone this repository. 

# Commands:

A brief list of accepted commands (until this point) of apllication. 

## save 

Save a url using the command `save`, followed by pocket name (the file name), the url, and a optional description for the url. 

```bash
cargo run save "books" "https://books.com" "a shop of books" 
```
if the pocket does not exists, it will be created. If the pocked exists, the url is saved on it.

## show 

Show all urls from a pocket using the command `show`.
```bash
cargo run show "books"
```
It'll print all url from "books".

## help 

The `help` command will show a list of available command on terminal for quick access. 

```bash
cargo run help
```
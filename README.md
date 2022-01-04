# Book Catalog REST API

A small book catalog REST API written in rust with the help of warp, tokio and diesel

## Description

My slightly modified/updated version from the one presented in this article by [Szymon Gibała](https://sgibala.com). I made some slight changes to the project layout out of personal preference and used the [diesel::r2d2::Builder](https://docs.diesel.rs/master/diesel/r2d2/struct.Builder.html) instead of the more generic (and 100% sufficient) connection style used in the tutorial.

I also added cargo-husky to run some basic checks once you start committing stuff and itconfig for easier handling of configuration variables.

## Features

- warp + tokio + diesel
- cargo-husky integration
- itconfig

# Project setup

- clone repository
- make sure you either have postgres installed locally or via docker
- install the diesel cli via `cargo install diesel_cli` (see [diesel docs](https://diesel.rs/guides/getting-started) if you need more information)
- configure the required enviroment variables (see [main.rs -> itconfig](src/main.rs))
- run `diesel setup --database-url='postgres://<user>:<password>@<host>:<port>/<database>'` to create the database and the required table
- run `cargo run`
  

## Acknowledgements

- [Building Rust Web API with Warp and Diesel](https://sgibala.com/01-01-rust-api-with-warp-and-diesel/) by [Szymon Gibała](https://sgibala.com)
- [the original repo](https://github.com/Szymongib/rust-api-warp-and-diesel)

## License

[Apache 2.0](https://choosealicense.com/licenses/apache-2.0/)

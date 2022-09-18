# What is "Cargo"?
It's the package manager of Rust language, and it's already built-in! It's also the build system of any project made with Rust.

# Main commands of Cargo
*  *cargo* new - Create a new project
*  *cargo* init - Create a new project *inside* an existing directory
*  *cargo* build - Build the project
*  *cargo* run - Run the project
*  *cargo* update - Update project dependencies
*  *cargo* test - Run tests
*  *cargo* bench - Run benchmarks
*  *cargo* doc - Generate the project documentation via *rustdoc*
*  *cargo* check - Analyze the project to see if it has any errors, without building it
*  *cargo* login - Login to *crates.io* with the API token
*  *cargo* package - Make the local crate uploadable to *crates.io*
*  *cargo* publish - Upload the crate to *crates.io*
*  *cargo* install - Install a Rust binary
*  *cargo* uninstall - Uninstall a Rust binary

# Wait, but what is a "Crate"?
A Crate is a *package* (remember that Cargo is the *package* manager!) written in Rust, and it can be shared to the internet via *"crates.io"*.
It's normally named with *snake_case*, but it's possible to see a crate named with *kebab-case*.
All installed crates are stored in the *.cargo/bin* directory.
A Crate can be *binary* type or *library* type. 

## Binary Crate
It's created via *"cargo new crate_name --bin"* or *"cargo new create_name"*. You can also create it by *"cargo new"* and then *"cargo run"* commands.

The binary crate implicit root (or entry point) will be called, by default, *"main.rs"* and it'll be inside a folder called *"src"*, where it's supposed to contain all the source code of a Crate.

In the same directory as the folder "src", a file *"Cargo.toml"* (that is a configuration file) will be created. This file will contain all the metadata needed to compile your binary crate.

## Library Crate
It's created via *"cargo new crate_name --lib"*. You can also create it by *"cargo new"* and then *"cargo test"* commands.

The library crate implicit root (or entry point) will be called, by default, *"lib.rs"* and it'll be inside a folder called *"src"*, where it's supposed to contain all the source code of a Crate.

In the same directory as the folder "src", a file *"Cargo.toml"* (that is a configuration file) will be created. This file will contain all the metadata needed to compile your binary crate.

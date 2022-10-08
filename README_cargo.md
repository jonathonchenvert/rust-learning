# RUST - USING CARGO

## Creating a project with Cargo 

Similar to virtualenv (or venv) in Python3, Rust can have its own development environment called `projects`. To invoke a new project, run the following:

```bash
cargo new <project_name>
# e.g.
cargo new hello_cargo
```

The following activities will take place:
- Creates a new project directory 
- Creates a `src/` directory with a `main.rs` file
- Creates a `Cargo.toml` file.
  - TOML -> Tom's Obvious, Minimal Language
- Initialize a Git repository with a `.gitignore` file (if not already in a Git repository)

## Initializing an existing project with Cargo

Converting an existing project to use Cargo can be done in the following order:
- Move existing project code into the `src/` directory 
- Create a `Cargo.toml` file in the parent project directory with the appropriate configurations

## Building and Running Cargo Projects

### Build
To build the Cargo project, one can run the following command in the project's directory:
```bash
cargo build
```

The `Cargo.lock` file created is similar to Python 3's `pip freeze > requirements.txt` command, where it keeps track of the exact versions of the dependencies in the project. 

### Run / Execution
To run the executable for the Cargo project that was built, run the following command:
```
# Running the project directly
./target/debug/<cargo_project_name> # macOS / Linux
.\target\debug\hello_cargo.exe # Windows

# Running the project with cargo
cargo run
```

`cargo run` can also rebuild the project if it detects any changes to the source files.

## Compiling Checks

The `cargo check` command can check and verify the code written can compile but won't produce an executable.

## Build for Release

Building for releasing the final product to an end user can be done with the following command:
```bash
cargo build --release
```

This compiles and builds the project with any optimizations. The executables will then be created in `target/release` instead of `target/debug`. The optimizations increase the Rust executions faster, but generally takes more time to compile. 

If it's desired to compile quickly and more often for development purposes, `cargo build` or `cargo check` will be better / quicker options.

## Updating Crates with Cargo
When updating a crate (package dependency), Cargo provides a command for updating the crate, which ignores the `Cargo.lock` file and find all the latest versions that fit in the project's specifications in the `Cargo.toml` file.

```bash
cargo update
```

This will also update the `Cargo.lock` file to denote the updated version to use.

##### [back to parent readme](../README.md)
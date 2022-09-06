# Jabberwock

**Jabberwock** is a simple static site generator written entirely in Rust.
It uses [Hatter](https://github.com/xvxx/hatter) as its templating language, and aims to be highly configurable and extendable using file pattern-based rules and Hatter's API.

At its very core, **Jabberwock** simply takes Hatter files from an input directory, transpiles them into HTML files, and writes the results to an output directory.

Additionally, a few other useful features such as transpiling markdown, including templates, and loading variables from TOML files are also included.
Since **Jabberwock** is designed to be as minimal as possible, these features can be disabled if desired.

# Installation

Unlike some of its contemporaries, **Jabberwock** is designed as a library rather than an executable or command-line application. It can be added to an existing project, or used in a new one.

**Jabberwock** can be included as a dependency by adding the following line under the `[dependencies]` section of a project's `Cargo.toml` file:
```toml
jabberwock = "0.1.0"
```

The `markdown`, `templates`, and `variables` features are enabled by default. If desired, these can be disabled:
```toml
jabberwock = { version = "0.1.0", default-features = false }
```

# Basics

HTML files can be generated from Hatter files using the `build()` function:
```rust
use jabberwock;

fn main()
{
    jabberwock::build().unwrap();
}
```
By default, **Jabberwock** looks for Hatter files inside the input directory, which is `in/` (relative to the project directory), and writes the output to the output directory, which is `out/` (relative to the project directory).

The result of the `build()` function will contain an error if something went wrong during the process, and nothing otherwise.

If desired, **Jabberwock**'s `Config` struct allows using custom configuration settings, including custom input/output directories, extra Hatter functions, and custom file rules:
```rust
use jabberwock;
use jabberwock::Config;
use jabberwock::FileRule;

fn main()
{
    let mut config = Config::new(); // or Config::empty() for an empty configuration with no default settings
    
    config.output_dir = String::from("site"); // change the output dir to "site/"
    config.set_file_rule("**/*.jpg", FileRule::Ignore); // ignore any JPEG files in the input directory and its subdirectories
    config.env.set("year", "2022"); // set variables in the top-level environment
    // add other custom settings if desired

    jabberwock::build_with(&mut config).unwrap();
}
```

Further information is available in **Jabberwock**'s [documentation](https://docs.rs/jabberwock/latest/jabberwock) and [wiki](https://github.com/Carnagion/jabberwock/wiki).

# Licensing

Any of the following two licenses can be selected when using **Jabberwock**:
- MIT (http://opensource.org/licenses/MIT)
- Apache 2.0 (https://opensource.org/licenses/Apache-2.0)
# Jabberwock

**Jabberwock** is a simple static site generator written entirely in Rust, using [Hatter](https://github.com/xvxx/hatter) as its templating language.
It aims to be highly configurable and modular using a system similar to that of [Metalsmith](https://github.com/metalsmith/metalsmith), while still being blazing fast and memory-safe.

# Installation

Unlike some of its contemporaries, **Jabberwock** is designed as a library rather than an executable or command-line application. It can be added to an existing project, or used in a new one.

**Jabberwock** can be included as a dependency by adding the following line under the `[dependencies]` section of a project's `Cargo.toml` file:
```toml
jabberwock = "0.1.0"
```

The `copy`, `markdown`, `templates`, `toml`, and `transpile` features are enabled by default. These enable the asset copier, Markdown transpiler, Hatter template transpiler, TOML transpiler, and main Hatter transpiler operations respectively.
If desired, they can be disabled:
```toml
jabberwock = { version = "0.1.0", default-features = false }
```

# Basics

**Jabberwock** uses a very simple process:
- Read files from an input directory as bytes and a path
- Perform one or more operations on the files' contents and paths
- Write the files' contents to the paths relative to an output directory

Commonly-used operations for static sites, such as transpiling Markdown, including templates, loading variables from TOML files, and so on are provided by default, but can be removed by disabling the respective features.

For example, the following code reads Hatter files from the input directory, transpiles them to HTML, and writes the results to the output directory:
```rust
use jabberwock::Generator;
use jabberwock::builtin::HatterTranspiler;

fn main() {
    println!("{:?}", generate());
}

fn generate() -> Result<(), String> {
    let mut generator = Generator::source("in/")?;  // read input files from "./in/"
    generator.apply(HatterTranspiler::new())?;      // transpile Hatter files to HTML files
    generator.destination("out/")                   // write output files  to "./out/"
}
```

The modular system allows chaining even more complex operations together.
For example, the following code transpiles Hatter files, adds Hatter functions to load variables from TOML, include Hatter files as templates, and include Markdown file contents as HTML, and also copies CSS files to the output:
```rust
use jabberwock::Generator;
use jabberwock::builtin::*;

fn main() {
    println!("{:?}", generate());
}

fn generate() -> Result<(), String> {
    let mut copier = AssetCopier::new();
    copier.copy("css/", "css/");                        // set up copier to copy all CSS files from "./css" to "./out/css"
    
    let mut generator = Generator::source("in/")?;      // read input files from "./in/"
    generator.apply(copier)?                            // include CSS files in the output
        .apply(MarkdownTranspiler::source("md/"))?      // add Hatter function to include transpiled Markdown from "./md/" inside Hatter files
        .apply(TemplateTranspiler::source("tmpl/"))?    // add Hatter function to include transpiled templates from "./tmpl/" inside Hatter files
        .apply(TomlTranspiler::source("vars/"))?        // add Hatter function to load TOML from "./vars/" as variables inside Hatter files
        .apply(HatterTranspiler::new())?;               // transpile Hatter files to HTML files
    generator.destination("out/")                       // write output files  to "./out/"
}
```

Further information is available in **Jabberwock**'s [documentation](https://docs.rs/jabberwock/latest/jabberwock).

# Licensing

Any of the following two licenses can be selected when using **Jabberwock**:
- MIT (http://opensource.org/licenses/MIT)
- Apache 2.0 (https://opensource.org/licenses/Apache-2.0)
use hatter::Env;

#[cfg(any(feature = "markdown", feature = "templates", feature = "variables"))]
use crate::builtin;

/// Specifies what to do with a file that is matched by a glob pattern.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FileRule
{
    /// File contents will be transpiled into HTML and written to the output directory, preserving directory structure.
    Transpile,
    /// File contents will be copied to the output directory, preserving directory structure.
    Copy,
    /// File will be ignored and will not be included in the output directory.
    #[default]
    Ignore,
}

/// Used when building. Contains the top-level Hatter [Env], as well as rules for each input file based on Unix-style glob patterns.
#[derive(Debug)]
pub struct Config
{
    /// The top-level Hatter environment to use when building.
    pub env: Env,

    /// The path to the input directory.
    pub input_dir: String,

    /// The path to the output directory.
    pub output_dir: String,

    file_rules: Vec<(String, FileRule)>,
}

impl Config
{
    /// A new configuration with the default file rules and input and output directories, same as [Config::default()].
    pub fn new() -> Config
    {
        Config::default()
    }

    /// An empty configuration with the default input and output directories, but no file rules.
    ///
    /// This configuration will do absolutely nothing, and is intended to be used only when specifying a build configuration with custom file rules.
    /// For the vast majority of cases, [Config::new()] or [Config::default()] should be used instead.
    pub fn empty() -> Config
    {
        Config
        {
            env: Env::new(),
            input_dir: String::from("in"),
            output_dir: String::from("out"),
            file_rules: vec![],
        }
    }

    /// Returns all file rules specified in a configuration.
    pub fn file_rules(&self) -> &Vec<(String, FileRule)>
    {
        &self.file_rules
    }

    /// Sets a rule for all files matched by a pattern, overriding any previously defined rules for them.
    pub fn set_file_rule(&mut self, pattern: impl Into<String>, rule: FileRule)
    {
        self.file_rules.push((pattern.into(), rule));
    }
}

impl Default for Config
{
    /// The default configuration.
    ///
    /// All files in the input directory (`in/`) will be copied to the output directory (`out/`), except for `.hat` files, which will be transpiled to `.html` files.
    /// Additionally, enabling certain features also causes all files in the following directories (relative to the input directory) to be ignored (including `.hat` files):
    /// - `md/` (if the `markdown` feature is enabled)
    /// - `tmpl/` (if the `templates` feature is enabled)
    /// - `vars/` (if the `variables` feature is enabled)
    fn default() -> Self
    {
        let mut config =  Config
        {
            env: Env::new(),
            input_dir: String::from("in"),
            output_dir: String::from("out"),
            file_rules: vec![],
        };

        // Copy all files in the input directory
        config.set_file_rule("**/*", FileRule::Copy);

        // Transpile all .hat files in the input directory
        config.set_file_rule("**/*.hat", FileRule::Transpile);

        #[cfg(feature = "markdown")]
        {
            config.env.set(builtin::MARKDOWN_DIR_VAR, "md");
            config.env.set("content", builtin::content);
            config.set_file_rule("md/**/*", FileRule::Ignore);
        }

        #[cfg(feature = "templates")]
        {
            config.env.set(builtin::TEMPLATES_DIR_VAR, "tmpl");
            config.env.set("include", builtin::include);
            config.set_file_rule("tmpl/**/*", FileRule::Ignore);
        }

        #[cfg(feature = "variables")]
        {
            config.env.set(builtin::VARIABLES_DIR_VAR, "vars");
            config.env.set("load", builtin::load);
            config.set_file_rule("vars/**/*", FileRule::Ignore);
        }

        config
    }
}
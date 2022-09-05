use std::path::Path;

use glob::Pattern;

use hatter::Env;

use crate::builtin;

/// Specifies what to do with a file that is matched by a pattern.
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

    file_rules: Vec<(Pattern, FileRule)>,
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
    pub fn file_rules(&self) -> &Vec<(Pattern, FileRule)>
    {
        &self.file_rules
    }

    /// Finds the latest specified file rule for a path, or the default if the path was not matched by any patterns.
    pub fn get_file_rule(&self, path: impl AsRef<Path>) -> FileRule
    {
        *self.file_rules
            .iter()
            .rev()
            .find(|(pattern, _)| pattern.matches_path(path.as_ref()))
            .map(|(_, rule)| rule)
            .unwrap_or(&FileRule::default())
    }

    /// Sets a rule for all files matched by a pattern, overriding any previously defined rules for them.
    pub fn set_file_rule(&mut self, pattern: impl Into<Pattern>, rule: FileRule)
    {
        self.file_rules.push((pattern.into(), rule));
    }
}

impl Default for Config
{
    /// The default configuration.
    ///
    /// Consists of the following directories and rules:
    /// - All files in the input directory (`in/`) will be copied to the output directory (`out/`)...
    /// - ...except for `.hat` files, which will be transpiled to `.html` files...
    /// - ..and files in the templates (`in/tmpl/`), variables (`in/vars/`), and markdown (`in/md/`) directories, which will be ignored (including Hatter files).
    fn default() -> Self
    {
        let mut config =  Config
        {
            env: Env::new(),
            input_dir: String::from("in"),
            output_dir: String::from("out"),
            file_rules: vec![],
        };

        config.env.set(builtin::MARKDOWN_DIR_VAR, "md");
        config.env.set("content", builtin::content);

        config.env.set(builtin::TEMPLATES_DIR_VAR, "tmpl");
        config.env.set("include", builtin::include);

        config.env.set(builtin::VARIABLES_DIR_VAR, "vars");
        config.env.set("load", builtin::load);

        // Copy all files in the input directory
        config.set_file_rule(Pattern::new("in/**/*").unwrap_or_default(), FileRule::Copy);

        // Transpile all .hat files in the input directory
        config.set_file_rule(Pattern::new("in/**/*.hat").unwrap(), FileRule::Transpile);

        // Ignore all files in the markdown, templates, and variables directories (including .hat files)
        config.set_file_rule(Pattern::new("in/md/*").unwrap(), FileRule::Ignore);
        config.set_file_rule(Pattern::new("in/tmpl/*").unwrap(), FileRule::Ignore);
        config.set_file_rule(Pattern::new("in/vars/*").unwrap(), FileRule::Ignore);

        config
    }
}
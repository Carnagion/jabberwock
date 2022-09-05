use std::path::Path;

use glob::Pattern;

use hatter::Env;

use crate::builtin;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FileRule
{
    Transpile,
    Copy,
    #[default]
    Ignore,
}

#[derive(Debug)]
pub struct Config
{
    pub env: Env,
    file_rules: Vec<(Pattern, FileRule)>,
    input_dir: String,
    output_dir: String,
}

impl Config
{
    pub fn new() -> Config
    {
        Config
        {
            env: Env::new(),
            file_rules: vec![],
            input_dir: String::from("in"),
            output_dir: String::from("out"),
        }
    }
    
    pub fn file_rules(&self) -> &Vec<(Pattern, FileRule)>
    {
        &self.file_rules
    }

    pub fn get_file_rule(&self, path: impl AsRef<Path>) -> FileRule
    {
        *self.file_rules
            .iter()
            .rev()
            .find(|(pattern, _)| pattern.matches_path(path.as_ref()))
            .map(|(_, rule)| rule)
            .unwrap_or(&FileRule::default())
    }

    pub fn set_file_rule(&mut self, pattern: impl Into<Pattern>, rule: FileRule)
    {
        self.file_rules.push((pattern.into(), rule));
    }

    pub fn get_input_dir(&self) -> &String
    {
        &self.input_dir
    }

    pub fn set_input_dir(&mut self, dir: impl Into<String>)
    {
        self.input_dir = dir.into()
    }

    pub fn get_output_dir(&self) -> &String
    {
        &self.output_dir
    }

    pub fn set_output_dir(&mut self, dir: impl Into<String>)
    {
        self.output_dir = dir.into()
    }
}

impl Default for Config
{
    fn default() -> Self
    {
        let mut config = Config::new();

        config.env.set(builtin::MARKDOWN_DIR_VAR, "md");
        config.env.set("content", builtin::content);

        config.env.set(builtin::TEMPLATES_DIR_VAR, "tmpl");
        config.env.set("include", builtin::include);

        config.env.set(builtin::VARIABLES_DIR_VAR, "vars");
        config.env.set("load", builtin::load);

        // Copy all files in the input directory
        config.set_file_rule(Pattern::new("in/**/*").unwrap(), FileRule::Copy);

        // Transpile all .hat files in the input directory
        config.set_file_rule(Pattern::new("in/**/*.hat").unwrap(), FileRule::Transpile);

        // Ignore all files in the markdown, templates, and variables directories (including .hat files)
        config.set_file_rule(Pattern::new("in/md/*").unwrap(), FileRule::Ignore);
        config.set_file_rule(Pattern::new("in/tmpl/*").unwrap(), FileRule::Ignore);
        config.set_file_rule(Pattern::new("in/vars/*").unwrap(), FileRule::Ignore);

        config
    }
}
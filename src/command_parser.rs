use once_cell::sync::Lazy;
use regex::Regex;

/// A regular expression for analyzing the command.
static COMMAND_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^/(?P<command>[^#\s]+)(\s+(?P<args>[^#]+))?(\s*#(?P<comment>.+))?").unwrap()
});

/// Command arguments.
pub struct CommandInfo<'ctx> {
    pub command: &'ctx str,
    pub args: Option<&'ctx str>,
    pub comment: Option<&'ctx str>
}

/// Tries to parse a command line.
pub fn parse_command<'ctx>(content: &'ctx str) -> Option<CommandInfo<'ctx>> {
    // Check the first character in order to avoid using regex again and again.
    if content.len() == 0 || &content[..1] != "/" {
        None
    }
    else {
        let parsed = COMMAND_REGEX.captures(content)?;

        let command = parsed.name("command").unwrap().as_str();
        let args = parsed.name("args").map(|parsed| parsed.as_str());
        let comment = parsed.name("comment").map(|parsed| parsed.as_str());

        Some(CommandInfo { command, args, comment })
    }
}
